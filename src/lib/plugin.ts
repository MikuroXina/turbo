import { Exports, Imports, createRuntime } from "./runtime";
import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";

const imports: Imports = {
    log(message) {
        console.log(message);
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
