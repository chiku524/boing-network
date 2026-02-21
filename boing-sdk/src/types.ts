/**
 * Types for Boing RPC results and params.
 */

/** Chain height (block number). */
export type ChainHeight = number;

/** Balance and stake are u128 as decimal strings. */
export interface AccountBalance {
  balance: string;
}

export interface AccountState {
  balance: string;
  nonce: number;
  stake: string;
}

export interface BlockHeader {
  parent_hash: string;
  height: number;
  timestamp: number;
  proposer: string;
  tx_root: string;
  state_root: string;
}

export interface Block {
  header: BlockHeader;
  transactions: unknown[];
}

export interface AccountProof {
  proof: string;
  root: string;
  value_hash: string;
}

export interface VerifyProofResult {
  valid: boolean;
}

export interface SimulateResult {
  gas_used: number;
  success: boolean;
  error?: string;
}

export interface SubmitTransactionResult {
  tx_hash: string;
}

export interface RegisterDappResult {
  registered: true;
  contract: string;
  owner: string;
}

export interface SubmitIntentResult {
  intent_id: string;
}

/** QA pre-flight result. */
export type QaCheckResult = 'allow' | 'reject' | 'unsure';

export interface QaCheckResponse {
  result: QaCheckResult;
  rule_id?: string;
  message?: string;
}

export interface FaucetResult {
  ok: true;
  amount: number;
  to: string;
  message: string;
}

/** JSON-RPC 2.0 response. */
export interface JsonRpcResponse<T = unknown> {
  jsonrpc: '2.0';
  id: number | string | null;
  result?: T;
  error?: {
    code: number;
    message: string;
    data?: unknown;
  };
}
