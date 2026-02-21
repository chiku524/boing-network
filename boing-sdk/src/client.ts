/**
 * Boing JSON-RPC client — typed methods for all node RPCs.
 */

import { BoingRpcError } from './errors.js';
import type {
  AccountBalance,
  AccountProof,
  AccountState,
  Block,
  FaucetResult,
  QaCheckResponse,
  RegisterDappResult,
  SimulateResult,
  SubmitIntentResult,
  SubmitTransactionResult,
  VerifyProofResult,
} from './types.js';
import { ensureHex, validateHex32 } from './hex.js';

const DEFAULT_RPC_ID = 1;
const DEFAULT_TIMEOUT_MS = 30_000;

export interface BoingClientConfig {
  baseUrl: string;
  /** Optional fetch implementation (e.g. for Node or custom headers). */
  fetch?: typeof fetch;
  /** Request timeout in ms. Default 30000. Set 0 to disable. */
  timeoutMs?: number;
}

/**
 * HTTP JSON-RPC client for a Boing node.
 * All methods return typed results; on RPC error they throw BoingRpcError (with code, message, method, and optional data).
 */
export class BoingClient {
  private readonly baseUrl: string;
  private readonly fetchImpl: typeof fetch;
  private readonly timeoutMs: number;
  private id = DEFAULT_RPC_ID;

  constructor(config: string | BoingClientConfig) {
    if (typeof config === 'string') {
      this.baseUrl = config.replace(/\/$/, '');
      this.fetchImpl = globalThis.fetch;
      this.timeoutMs = DEFAULT_TIMEOUT_MS;
    } else {
      this.baseUrl = config.baseUrl.replace(/\/$/, '');
      this.fetchImpl = config.fetch ?? globalThis.fetch;
      this.timeoutMs = config.timeoutMs ?? DEFAULT_TIMEOUT_MS;
    }
  }

  private async request<T>(method: string, params: unknown[] = []): Promise<T> {
    const body = {
      jsonrpc: '2.0',
      id: this.id++,
      method,
      params,
    };
    const controller = this.timeoutMs > 0 ? new AbortController() : null;
    const timeoutId = controller
      ? setTimeout(() => controller.abort(), this.timeoutMs)
      : null;
    try {
      const res = await this.fetchImpl(this.baseUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
        signal: controller?.signal,
      });
      if (!res.ok) {
        throw new BoingRpcError(
          -32000,
          `HTTP ${res.status}: ${res.statusText}`,
          undefined,
          method
        );
      }
      const json = (await res.json()) as {
        jsonrpc?: string;
        id?: number;
        result?: T;
        error?: { code: number; message: string; data?: unknown };
      };
      if (json.error != null) {
        throw new BoingRpcError(
          json.error.code,
          json.error.message,
          json.error.data,
          method
        );
      }
      if (!('result' in json)) {
        throw new BoingRpcError(
          -32000,
          'Invalid RPC response: no result or error',
          undefined,
          method
        );
      }
      return json.result as T;
    } catch (err) {
      if (err instanceof BoingRpcError) throw err;
      if (err instanceof Error && err.name === 'AbortError') {
        throw new BoingRpcError(
          -32000,
          `Request timed out after ${this.timeoutMs}ms`,
          undefined,
          method
        );
      }
      throw new BoingRpcError(
        -32000,
        err instanceof Error ? err.message : String(err),
        undefined,
        method
      );
    } finally {
      if (timeoutId != null) clearTimeout(timeoutId);
    }
  }

  /** Current chain height (tip block number). */
  async chainHeight(): Promise<number> {
    return this.request<number>('boing_chainHeight', []);
  }

  /** Get spendable balance for an account. Params: 32-byte account ID (hex). */
  async getBalance(hexAccountId: string): Promise<AccountBalance> {
    const hex = validateHex32(hexAccountId);
    return this.request<AccountBalance>('boing_getBalance', [hex]);
  }

  /** Get full account state (balance, nonce, stake). Params: 32-byte account ID (hex). */
  async getAccount(hexAccountId: string): Promise<AccountState> {
    const hex = validateHex32(hexAccountId);
    return this.request<AccountState>('boing_getAccount', [hex]);
  }

  /** Get block by height. Returns null if not found. */
  async getBlockByHeight(height: number): Promise<Block | null> {
    return this.request<Block | null>('boing_getBlockByHeight', [height]);
  }

  /** Get block by hash. Params: 32-byte block hash (hex). */
  async getBlockByHash(hexBlockHash: string): Promise<Block | null> {
    const hex = validateHex32(hexBlockHash);
    return this.request<Block | null>('boing_getBlockByHash', [hex]);
  }

  /** Get Merkle proof for an account. Params: 32-byte account ID (hex). */
  async getAccountProof(hexAccountId: string): Promise<AccountProof> {
    const hex = validateHex32(hexAccountId);
    return this.request<AccountProof>('boing_getAccountProof', [hex]);
  }

  /** Verify an account Merkle proof. Params: hex proof, hex state root. */
  async verifyAccountProof(hexProof: string, hexStateRoot: string): Promise<VerifyProofResult> {
    return this.request<VerifyProofResult>('boing_verifyAccountProof', [
      ensureHex(hexProof),
      ensureHex(hexStateRoot),
    ]);
  }

  /** Simulate a transaction without applying it. Params: hex-encoded signed transaction. */
  async simulateTransaction(hexSignedTx: string): Promise<SimulateResult> {
    const hex = ensureHex(hexSignedTx);
    return this.request<SimulateResult>('boing_simulateTransaction', [hex]);
  }

  /**
   * Submit a signed transaction to the mempool.
   * The hex_signed_tx must be hex-encoded bincode-serialized SignedTransaction (from Rust/CLI or future signer).
   */
  async submitTransaction(hexSignedTx: string): Promise<SubmitTransactionResult> {
    const hex = ensureHex(hexSignedTx);
    return this.request<SubmitTransactionResult>('boing_submitTransaction', [hex]);
  }

  /** Register a dApp for incentive tracking. Params: 32-byte hex contract id, 32-byte hex owner id. */
  async registerDappMetrics(hexContract: string, hexOwner: string): Promise<RegisterDappResult> {
    return this.request<RegisterDappResult>('boing_registerDappMetrics', [
      validateHex32(hexContract),
      validateHex32(hexOwner),
    ]);
  }

  /** Submit a signed intent. Params: hex-encoded signed intent. */
  async submitIntent(hexSignedIntent: string): Promise<SubmitIntentResult> {
    return this.request<SubmitIntentResult>('boing_submitIntent', [ensureHex(hexSignedIntent)]);
  }

  /**
   * Pre-flight QA check for a deployment (no submit).
   * Params: hex bytecode; optionally purpose_category and description_hash.
   * Returns allow | reject | unsure; when reject, rule_id and message are set.
   */
  async qaCheck(
    hexBytecode: string,
    purposeCategory?: string,
    descriptionHash?: string
  ): Promise<QaCheckResponse> {
    const hex = ensureHex(hexBytecode);
    const params: string[] = [hex];
    if (purposeCategory != null) {
      params.push(purposeCategory);
      if (descriptionHash != null) params.push(descriptionHash);
    }
    return this.request<QaCheckResponse>('boing_qaCheck', params);
  }

  /** Request testnet BOING (only when node is started with --faucet-enable). Params: 32-byte account ID (hex). Rate limited per account. */
  async faucetRequest(hexAccountId: string): Promise<FaucetResult> {
    const hex = validateHex32(hexAccountId);
    return this.request<FaucetResult>('boing_faucetRequest', [hex]);
  }
}
