/* tslint:disable */
/* eslint-disable */

export class Camera {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static new(): Camera;
    render(world: HittableList): string[];
    set_lookat(u: number, v: number, w: number): void;
    set_lookfrom(u: number, v: number, w: number): void;
    set_vup(u: number, v: number, w: number): void;
    aspect_ratio: number;
    image_width: number;
    max_depth: number;
    samples_per_pixel: number;
    vfov: number;
}

export class HittableList {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    add_glass_sphere(x: number, y: number, z: number, radius: number, idr: number): void;
    add_hollow_glass_sphere(x: number, y: number, z: number, radius: number, thickness: number): void;
    add_matte_sphere(x: number, y: number, z: number, radius: number, r: number, g: number, b: number): void;
    add_metal_sphere(x: number, y: number, z: number, radius: number, r: number, g: number, b: number, fuzz: number): void;
    clear(): void;
    static new(): HittableList;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_camera_free: (a: number, b: number) => void;
    readonly __wbg_get_camera_aspect_ratio: (a: number) => number;
    readonly __wbg_get_camera_image_width: (a: number) => number;
    readonly __wbg_get_camera_max_depth: (a: number) => number;
    readonly __wbg_get_camera_samples_per_pixel: (a: number) => number;
    readonly __wbg_get_camera_vfov: (a: number) => number;
    readonly __wbg_hittablelist_free: (a: number, b: number) => void;
    readonly __wbg_set_camera_aspect_ratio: (a: number, b: number) => void;
    readonly __wbg_set_camera_image_width: (a: number, b: number) => void;
    readonly __wbg_set_camera_max_depth: (a: number, b: number) => void;
    readonly __wbg_set_camera_samples_per_pixel: (a: number, b: number) => void;
    readonly __wbg_set_camera_vfov: (a: number, b: number) => void;
    readonly camera_new: () => number;
    readonly camera_render: (a: number, b: number) => [number, number];
    readonly camera_set_lookat: (a: number, b: number, c: number, d: number) => void;
    readonly camera_set_lookfrom: (a: number, b: number, c: number, d: number) => void;
    readonly camera_set_vup: (a: number, b: number, c: number, d: number) => void;
    readonly hittablelist_add_glass_sphere: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly hittablelist_add_hollow_glass_sphere: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly hittablelist_add_matte_sphere: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly hittablelist_add_metal_sphere: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
    readonly hittablelist_clear: (a: number) => void;
    readonly hittablelist_new: () => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
