import { PluginExports, extractSource } from "./exports";
import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";

const loadPlugins = async (setPlugins: (exports: PluginExports[]) => void): Promise<void> => {
    const raws = (await invoke("all_plugins")) as {
        source: string;
    }[];
    const loaded = await Promise.all(raws.map(({ source }) => extractSource(source)));
    for (const exports of loaded) {
        if (!exports.load) {
            throw new Error("load proc not found");
        }
        exports.load();
    }
    setPlugins(loaded);
};

const dropPlugins = (plugins: PluginExports[]) => {
    for (const exports of plugins) {
        if (!exports.unload) {
            throw new Error("load proc not found");
        }
        exports.unload();
    }
};

export const usePlugins = (): PluginExports[] => {
    const [plugins, setPlugins] = useState<PluginExports[]>([]);
    useEffect(() => {
        void loadPlugins(setPlugins);
        return () => dropPlugins(plugins);
    }, []);
    return plugins;
};
