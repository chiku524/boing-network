/**
 * RPC error with optional structured data (e.g. QA rejection rule_id and message).
 * When thrown from BoingClient, `method` is set to the RPC method that failed.
 */
export class BoingRpcError extends Error {
  constructor(
    public readonly code: number,
    message: string,
    public readonly data?: unknown,
    /** RPC method that failed (e.g. "boing_getBalance"). */
    public readonly method?: string
  ) {
    super(message);
    this.name = 'BoingRpcError';
    Object.setPrototypeOf(this, BoingRpcError.prototype);
  }

  /** Short string for logging: "BoingRpcError(code, method): message". */
  override toString(): string {
    const method = this.method ? ` ${this.method}` : '';
    return `BoingRpcError(${this.code}${method}): ${this.message}`;
  }

  /** True if this is a QA deployment rejection (-32050). */
  get isQaRejected(): boolean {
    return this.code === -32050;
  }

  /** True if deployment was referred to community QA pool (-32051). */
  get isQaPendingPool(): boolean {
    return this.code === -32051;
  }

  /** QA rejection details when code is -32050. */
  get qaData(): { rule_id: string; message: string } | undefined {
    if (this.code !== -32050 || !this.data || typeof this.data !== 'object') return undefined;
    const d = this.data as Record<string, unknown>;
    if (typeof d.rule_id === 'string' && typeof d.message === 'string') {
      return { rule_id: d.rule_id, message: d.message };
    }
    return undefined;
  }
}
