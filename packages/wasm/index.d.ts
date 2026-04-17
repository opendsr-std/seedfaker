export interface FakerOptions {
  seed?: string;
  locale?: string;
  tz?: string;
  since?: number;
  until?: number;
}

export interface RecordOptions {
  n?: number;
  ctx?: "strict" | "loose";
  corrupt?: "low" | "mid" | "high" | "extreme";
}

export class SeedFaker {
  /** Load WASM module. Must be called once before creating instances. */
  static init(): Promise<void>;

  constructor(opts?: FakerOptions);

  /** Generate a single field value. */
  field(name: string, opts?: Record<string, unknown>): string;

  /** Generate a single record as an object. */
  record(
    fields: string[],
    opts?: RecordOptions,
  ): Record<string, string>;

  /** Generate multiple records. */
  records(
    fields: string[],
    opts?: RecordOptions & { n?: number },
  ): Record<string, string>[];

  /** Validate field specs without generating. */
  validate(fields: string[], opts?: RecordOptions): void;

  /** All field names. */
  static fields(): string[];

  /** Algorithm fingerprint. */
  static fingerprint(): string;
}
