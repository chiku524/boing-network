//! Boing VM interpreter — deterministic stack machine.
//!
//! Executes bytecode with gas metering.

use boing_primitives::AccountId;
use boing_state::StateStore;

use super::bytecode::{gas, Opcode};
use super::vm::VmError;

/// Stack machine interpreter.
pub struct Interpreter {
    pub code: Vec<u8>,
    pub pc: usize,
    pub stack: Vec<[u8; 32]>,
    pub memory: Vec<u8>,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub return_data: Option<Vec<u8>>,
}

/// Storage interface for SLOAD/SSTORE.
pub trait StorageAccess {
    fn sload(&self, contract: AccountId, key: [u8; 32]) -> [u8; 32];
    fn sstore(&mut self, contract: AccountId, key: [u8; 32], value: [u8; 32]);
}

impl Interpreter {
    pub fn new(code: Vec<u8>, gas_limit: u64) -> Self {
        Self {
            code,
            pc: 0,
            stack: Vec::new(),
            memory: Vec::new(),
            gas_used: 0,
            gas_limit,
            return_data: None,
        }
    }

    fn spend_gas(&mut self, amount: u64) -> Result<(), VmError> {
        self.gas_used = self.gas_used.saturating_add(amount);
        if self.gas_used > self.gas_limit {
            return Err(VmError::OutOfGas);
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<[u8; 32], VmError> {
        self.stack.pop().ok_or(VmError::StackUnderflow)
    }

    fn push(&mut self, value: [u8; 32]) {
        self.stack.push(value);
    }

    fn ensure_memory(&mut self, offset: usize, size: usize) {
        let end = offset.saturating_add(size);
        if end > self.memory.len() {
            self.memory.resize(end, 0);
        }
    }

    fn u256_to_usize(v: &[u8; 32]) -> usize {
        let mut n: u64 = 0;
        for (i, &b) in v.iter().rev().take(8).enumerate() {
            n |= (b as u64) << (i * 8);
        }
        n as usize
    }

    fn add_u256(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let mut carry = 0u16;
        let mut out = [0u8; 32];
        for i in (0..32).rev() {
            let s = (a[i] as u16) + (b[i] as u16) + carry;
            out[i] = s as u8;
            carry = s >> 8;
        }
        out
    }

    fn sub_u256(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let mut borrow = 0i32;
        let mut out = [0u8; 32];
        for i in (0..32).rev() {
            let diff = (a[i] as i32) - (b[i] as i32) - borrow;
            borrow = if diff < 0 { 1 } else { 0 };
            out[i] = diff.wrapping_rem(256) as u8;
        }
        out
    }

    fn mul_u256(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let a64 = Self::u256_to_u64(a);
        let b64 = Self::u256_to_u64(b);
        Self::u64_to_u256(a64.saturating_mul(b64))
    }

    fn u256_to_u64(v: &[u8; 32]) -> u64 {
        let mut n: u64 = 0;
        for (i, &b) in v.iter().rev().take(8).enumerate() {
            n |= (b as u64) << (i * 8);
        }
        n
    }

    fn u64_to_u256(n: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&n.to_be_bytes());
        out
    }

    /// Execute until STOP or RETURN. Returns gas used.
    pub fn run<S: StorageAccess>(
        &mut self,
        contract: AccountId,
        calldata: &[u8],
        storage: &mut S,
    ) -> Result<u64, VmError> {
        self.ensure_memory(0, calldata.len());
        self.memory[..calldata.len()].copy_from_slice(calldata);

        while self.pc < self.code.len() {
            let op = self.code[self.pc];
            self.pc += 1;

            if let Some(push_len) = Opcode::push_size(op) {
                self.spend_gas(gas::PUSH)?;
                let len = push_len as usize;
                if self.pc + len > self.code.len() {
                    return Err(VmError::InvalidBytecode);
                }
                let mut val = [0u8; 32];
                let start = 32 - len;
                val[start..].copy_from_slice(&self.code[self.pc..self.pc + len]);
                self.stack.push(val);
                self.pc += len;
                continue;
            }

            let opcode = Opcode::from_byte(op).ok_or(VmError::InvalidBytecode)?;

            match opcode {
                Opcode::Stop => {
                    self.spend_gas(gas::STOP)?;
                    break;
                }
                Opcode::Add => {
                    self.spend_gas(gas::ADD)?;
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Self::add_u256(&a, &b));
                }
                Opcode::Sub => {
                    self.spend_gas(gas::SUB)?;
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Self::sub_u256(&a, &b));
                }
                Opcode::Mul => {
                    self.spend_gas(gas::MUL)?;
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Self::mul_u256(&a, &b));
                }
                Opcode::MLoad => {
                    self.spend_gas(gas::MLOAD)?;
                    let offset = Self::u256_to_usize(&self.pop()?);
                    self.ensure_memory(offset, 32);
                    let mut val = [0u8; 32];
                    val.copy_from_slice(&self.memory[offset..offset + 32]);
                    self.push(val);
                }
                Opcode::MStore => {
                    self.spend_gas(gas::MSTORE)?;
                    let offset = Self::u256_to_usize(&self.pop()?);
                    let value = self.pop()?;
                    self.ensure_memory(offset, 32);
                    self.memory[offset..offset + 32].copy_from_slice(&value);
                }
                Opcode::SLoad => {
                    self.spend_gas(gas::SLOAD)?;
                    let key = self.pop()?;
                    let value = storage.sload(contract, key);
                    self.push(value);
                }
                Opcode::SStore => {
                    self.spend_gas(gas::SSTORE)?;
                    let key = self.pop()?;
                    let value = self.pop()?;
                    storage.sstore(contract, key, value);
                }
                Opcode::Jump => {
                    self.spend_gas(gas::JUMP)?;
                    let dest = Self::u256_to_usize(&self.pop()?);
                    if dest >= self.code.len() {
                        return Err(VmError::InvalidJump);
                    }
                    self.pc = dest;
                }
                Opcode::JumpI => {
                    self.spend_gas(gas::JUMPI)?;
                    let dest = Self::u256_to_usize(&self.pop()?);
                    let cond = self.pop()?;
                    let is_nonzero = cond != [0u8; 32];
                    if is_nonzero && dest < self.code.len() {
                        self.pc = dest;
                    }
                }
                Opcode::Return => {
                    self.spend_gas(gas::RETURN)?;
                    let offset = Self::u256_to_usize(&self.pop()?);
                    let size = Self::u256_to_usize(&self.pop()?);
                    self.ensure_memory(offset, size);
                    self.return_data = Some(self.memory[offset..offset + size].to_vec());
                    break;
                }
                Opcode::Push1 | Opcode::Push32 => {
                    unreachable!("handled above")
                }
            }
        }

        Ok(self.gas_used)
    }
}

impl StorageAccess for StateStore {
    fn sload(&self, contract: AccountId, key: [u8; 32]) -> [u8; 32] {
        self.contract_storage
            .get(&(contract, key))
            .copied()
            .unwrap_or([0u8; 32])
    }

    fn sstore(&mut self, contract: AccountId, key: [u8; 32], value: [u8; 32]) {
        self.contract_storage.insert((contract, key), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_add_stop() {
        let mut state = StateStore::new();
        let contract = AccountId([1u8; 32]);
        state.insert(boing_primitives::Account {
            id: contract,
            state: boing_primitives::AccountState { balance: 0, nonce: 0, stake: 0 },
        });
        let bytecode = vec![
            0x60, 0x02, // PUSH1 2
            0x60, 0x03, // PUSH1 3
            0x01,       // ADD -> 5
            0x00,       // STOP
        ];
        state.set_contract_code(contract, bytecode.clone());
        let mut interpreter = Interpreter::new(bytecode, 1000);
        let gas = interpreter.run(contract, &[], &mut state).unwrap();
        assert!(gas > 0);
        assert_eq!(interpreter.stack.len(), 1);
        assert_eq!(interpreter.stack[0][31], 5); // low byte = 5
    }
}
