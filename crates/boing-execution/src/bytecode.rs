//! Boing VM bytecode — minimal stack-based instruction set.
//!
//! Opcodes inspired by EVM, simplified for determinism and auditability.

/// Single-byte opcodes.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    /// Halt execution (0x00)
    Stop = 0x00,
    /// Add top two stack values (0x01)
    Add = 0x01,
    /// Subtract (0x02)
    Sub = 0x02,
    /// Multiply (0x03)
    Mul = 0x03,
    /// Load from memory at offset (0x51)
    MLoad = 0x51,
    /// Store to memory (0x52)
    MStore = 0x52,
    /// Load from storage (0x54)
    SLoad = 0x54,
    /// Store to storage (0x55)
    SStore = 0x55,
    /// Push 1 byte immediate (0x60)
    Push1 = 0x60,
    /// Push 32 bytes (0x7f)
    Push32 = 0x7f,
    /// Pop and jump to offset (0x56)
    Jump = 0x56,
    /// Conditional jump (0x57)
    JumpI = 0x57,
    /// Return memory slice (0xf3)
    Return = 0xf3,
}

impl Opcode {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x00 => Some(Self::Stop),
            0x01 => Some(Self::Add),
            0x02 => Some(Self::Sub),
            0x03 => Some(Self::Mul),
            0x51 => Some(Self::MLoad),
            0x52 => Some(Self::MStore),
            0x54 => Some(Self::SLoad),
            0x55 => Some(Self::SStore),
            0x56 => Some(Self::Jump),
            0x57 => Some(Self::JumpI),
            0x60 => Some(Self::Push1),
            0x7f => Some(Self::Push32),
            0xf3 => Some(Self::Return),
            _ => None,
        }
    }

    pub fn push_size(b: u8) -> Option<u8> {
        if (0x60..=0x7f).contains(&b) {
            Some(b - 0x5f) // PUSH1 = 1, PUSH32 = 32
        } else {
            None
        }
    }
}

/// Gas cost per opcode (base costs).
pub mod gas {
    pub const STOP: u64 = 0;
    pub const ADD: u64 = 3;
    pub const SUB: u64 = 3;
    pub const MUL: u64 = 5;
    pub const MLOAD: u64 = 3;
    pub const MSTORE: u64 = 3;
    pub const SLOAD: u64 = 100;
    pub const SSTORE: u64 = 20_000;
    pub const PUSH: u64 = 3;
    pub const JUMP: u64 = 8;
    pub const JUMPI: u64 = 10;
    pub const RETURN: u64 = 0;
}
