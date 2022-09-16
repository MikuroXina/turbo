import { Exports, Imports, createRuntime } from "./runtime";
import type { KeyPath, PrimitiveParam, RenderObject, Shader } from "./runtime/types";
import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";

const store = new Map<string, PrimitiveParam>();
const renderObjects = new Map<string, RenderObject>();
const shaders = new Map<string, Shader>();

const imports: Imports = {
    log(message) {
        console.log(message);
    },
    getFromStore: (key: KeyPath) => store.get(key.join("/")) ?? null,
    registerRenderObject: (obj: RenderObject) => {
        if (renderObjects.has(obj.identifier)) {
            throw new Error(`render object identifier conflicted: ${obj.identifier}`);
        }
        renderObjects.set(obj.identifier, obj);
    },
    registerShader: (obj: Shader) => {
        if (shaders.has(obj.identifier)) {
            throw new Error(`shader identifier conflicted: ${obj.identifier}`);
        }
        shaders.set(obj.identifier, obj);
    },
};

const loadPlugins = async (setPlugins: (exports: Exports[]) => void): Promise<void> => {
    const raws = (await invoke("all_plugins")) as {
        wasm_binary: number[];
    }[];
    const loaded = await Promise.all(
        raws.map((raw) => createRuntime(new Uint8Array(raw.wasm_binary), imports)),
    );
    for (const exports of loaded) {
        if (!exports.load) {
            throw new Error("load proc not found");
        }
        exports.load();
    }
    setPlugins(loaded);
};

const dropPlugins = (plugins: Exports[]) => {
    for (const exports of plugins) {
        if (!exports.unload) {
            throw new Error("load proc not found");
        }
        exports.unload();
    }
};

export const usePlugins = (): Exports[] => {
    const [plugins, setPlugins] = useState<Exports[]>([]);
    useEffect(() => {
        void loadPlugins(setPlugins);
        return () => dropPlugins(plugins);
    }, []);
    return plugins;
};
