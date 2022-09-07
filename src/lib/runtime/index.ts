/* eslint-disable */
// ============================================= //
// WebAssembly runtime for TypeScript            //
//                                               //
// This file is generated. PLEASE DO NOT MODIFY. //
// ============================================= //
// deno-lint-ignore-file no-explicit-any no-unused-vars

import type * as types from "./types";

import { decode, encode } from "@msgpack/msgpack";

type FatPtr = bigint;

export type Imports = {
    log: (message: string) => void;
};

export type Exports = {
    load?: () => void;
    metadata?: () => types.Metadata;
    renderObjectFactory?: (
        format: types.FileFormat,
        data: Uint8Array,
    ) => types.Result<types.RenderObject, types.RenderObjectFactoryError>;
    unload?: () => void;
    metadataRaw?: () => Uint8Array;
    renderObjectFactoryRaw?: (format: Uint8Array, data: Uint8Array) => Uint8Array;
};

/**
 * Represents an unrecoverable error in the FP runtime.
 *
 * After this, your only recourse is to create a new runtime, probably with a different WASM plugin.
 */
export class FPRuntimeError extends Error {
    constructor(message: string) {
        super(message);
    }
}

/**
 * Creates a runtime for executing the given plugin.
 *
 * @param plugin The raw WASM plugin.
 * @param importFunctions The host functions that may be imported by the plugin.
 * @returns The functions that may be exported by the plugin.
 */
export async function createRuntime(
    plugin: ArrayBuffer,
    importFunctions: Imports,
): Promise<Exports> {
    const promises = new Map<FatPtr, ((result: FatPtr) => void) | FatPtr>();

    function createAsyncValue(): FatPtr {
        const len = 12; // std::mem::size_of::<AsyncValue>()
        const fatPtr = malloc(len);
        const [ptr] = fromFatPtr(fatPtr);
        const buffer = new Uint8Array(memory.buffer, ptr, len);
        buffer.fill(0);
        return fatPtr;
    }

    function interpretSign(num: number, cap: number) {
        if (num < cap) {
            return num;
        } else {
            return num - (cap << 1);
        }
    }

    function interpretBigSign(num: bigint, cap: bigint) {
        if (num < cap) {
            return num;
        } else {
            return num - (cap << 1n);
        }
    }

    function parseObject<T>(fatPtr: FatPtr): T {
        const [ptr, len] = fromFatPtr(fatPtr);
        const buffer = new Uint8Array(memory.buffer, ptr, len);
        // Without creating a copy of the memory, we risk corruption of any
        // embedded `Uint8Array` objects returned from `decode()` after `free()`
        // has been called :(
        const copy = new Uint8Array(len);
        copy.set(buffer);
        free(fatPtr);
        const object = decode(copy) as unknown as T;
        return object;
    }

    function promiseFromPtr(ptr: FatPtr): Promise<FatPtr> {
        const resultPtr = promises.get(ptr);
        if (resultPtr) {
            if (typeof resultPtr === "function") {
                throw new FPRuntimeError("Already created promise for this value");
            }

            promises.delete(ptr);
            return Promise.resolve(resultPtr);
        } else {
            return new Promise((resolve) => {
                promises.set(ptr, resolve as (result: FatPtr) => void);
            });
        }
    }

    function resolvePromise(asyncValuePtr: FatPtr, resultPtr: FatPtr) {
        const resolve = promises.get(asyncValuePtr);
        if (resolve) {
            if (typeof resolve !== "function") {
                throw new FPRuntimeError("Tried to resolve invalid promise");
            }

            promises.delete(asyncValuePtr);
            resolve(resultPtr);
        } else {
            promises.set(asyncValuePtr, resultPtr);
        }
    }

    function serializeObject<T>(object: T): FatPtr {
        return exportToMemory(encode(object));
    }

    function exportToMemory(serialized: Uint8Array): FatPtr {
        const fatPtr = malloc(serialized.length);
        const [ptr, len] = fromFatPtr(fatPtr);
        const buffer = new Uint8Array(memory.buffer, ptr, len);
        buffer.set(serialized);
        return fatPtr;
    }

    function importFromMemory(fatPtr: FatPtr): Uint8Array {
        const [ptr, len] = fromFatPtr(fatPtr);
        const buffer = new Uint8Array(memory.buffer, ptr, len);
        const copy = new Uint8Array(len);
        copy.set(buffer);
        free(fatPtr);
        return copy;
    }

    const { instance } = await WebAssembly.instantiate(plugin, {
        fp: {
            __fp_gen_log: (message_ptr: FatPtr) => {
                const message = parseObject<string>(message_ptr);
                importFunctions.log(message);
            },
        },
    });

    const getExport = <T>(name: string): T => {
        const exp = instance.exports[name];
        if (!exp) {
            throw new FPRuntimeError(`Plugin did not export expected symbol: "${name}"`);
        }
        return exp as unknown as T;
    };

    const memory = getExport<WebAssembly.Memory>("memory");
    const malloc = getExport<(len: number) => FatPtr>("__fp_malloc");
    const free = getExport<(ptr: FatPtr) => void>("__fp_free");

    return {
        load: instance.exports.__fp_gen_load as any,
        metadata: (() => {
            const export_fn = instance.exports.__fp_gen_metadata as any;
            if (!export_fn) return;

            return () => parseObject<types.Metadata>(export_fn());
        })(),
        renderObjectFactory: (() => {
            const export_fn = instance.exports.__fp_gen_render_object_factory as any;
            if (!export_fn) return;

            return (format: types.FileFormat, data: Uint8Array) => {
                const format_ptr = serializeObject(format);
                const data_ptr = serializeObject(data);
                return parseObject<
                    types.Result<types.RenderObject, types.RenderObjectFactoryError>
                >(export_fn(format_ptr, data_ptr));
            };
        })(),
        unload: instance.exports.__fp_gen_unload as any,
        metadataRaw: (() => {
            const export_fn = instance.exports.__fp_gen_metadata as any;
            if (!export_fn) return;

            return () => importFromMemory(export_fn());
        })(),
        renderObjectFactoryRaw: (() => {
            const export_fn = instance.exports.__fp_gen_render_object_factory as any;
            if (!export_fn) return;

            return (format: Uint8Array, data: Uint8Array) => {
                const format_ptr = exportToMemory(format);
                const data_ptr = exportToMemory(data);
                return importFromMemory(export_fn(format_ptr, data_ptr));
            };
        })(),
    };
}

function fromFatPtr(fatPtr: FatPtr): [ptr: number, len: number] {
    return [
        Number.parseInt((fatPtr >> 32n).toString()),
        Number.parseInt((fatPtr & 0xffff_ffffn).toString()),
    ];
}

function toFatPtr(ptr: number, len: number): FatPtr {
    return (BigInt(ptr) << 32n) | BigInt(len);
}
