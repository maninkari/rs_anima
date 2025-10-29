/* tslint:disable */
/* eslint-disable */
export function set_speed(speed: number): void;
export function set_show_longitude(show: boolean): void;
export function set_show_latitude(show: boolean): void;
export function set_show_tunnel(show: boolean): void;
export function set_num_polygons(num: number): void;
export function set_outside_view(outside: boolean): void;
export function start_simple_tunnel(canvas_id: string, a: number, b: number, r: number, polygon_radius: number, polygon_sides: number, num_polygons: number): void;
export class Lissajou3D {
  free(): void;
  constructor(a: number, b: number, r: number);
  readonly a: number;
  readonly b: number;
  readonly r: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly set_show_longitude: (a: number) => void;
  readonly set_show_latitude: (a: number) => void;
  readonly set_show_tunnel: (a: number) => void;
  readonly set_outside_view: (a: number) => void;
  readonly start_simple_tunnel: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly set_speed: (a: number) => void;
  readonly set_num_polygons: (a: number) => void;
  readonly __wbg_lissajou3d_free: (a: number, b: number) => void;
  readonly lissajou3d_new: (a: number, b: number, c: number) => number;
  readonly lissajou3d_a: (a: number) => number;
  readonly lissajou3d_b: (a: number) => number;
  readonly lissajou3d_r: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h17542a08b0794ff2: (a: number, b: number, c: number) => void;
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
