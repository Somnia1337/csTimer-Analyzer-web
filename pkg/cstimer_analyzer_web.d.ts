/* tslint:disable */
/* eslint-disable */
/**
 * Converts the markdown content to HTML, a
 * more time-efficient equivalent to marked.js.
 */
export function render_markdown(input: string): any;
/**
 * Initializes the analysis, parses options and sessions.
 */
export function init_analysis(options_txt: Uint8Array, data_txt: Uint8Array, canvas: HTMLCanvasElement): void;
/**
 * Provides information about the dataset and parsed options.
 */
export function analysis_info(): any;
/**
 * Provides number of sessions to be analyzed.
 */
export function get_session_count(): number;
/**
 * Provides the analysis for the session specified by JS.
 */
export function analyze_session(index: number): any;
/**
 * Provides debug information about analysis timings.
 */
export function get_timings(): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly render_markdown: (a: number, b: number) => any;
  readonly init_analysis: (a: number, b: number, c: number, d: number, e: any) => [number, number];
  readonly analysis_info: () => [number, number, number];
  readonly get_session_count: () => number;
  readonly analyze_session: (a: number) => [number, number, number];
  readonly get_timings: () => [number, number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
