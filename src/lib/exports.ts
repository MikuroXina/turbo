import type { FileHandleDef, ParamBase, RenderObjectDef, ShaderDef, SpeakerDef } from "./def";

import type { FileHandle } from "fs/promises";
import type { Param } from "./param";

declare const idNominal: unique symbol;
export type Identifier = string & { [idNominal]: never };

declare const verNominal: unique symbol;
export type Version = [number, number, number] & { [verNominal]: never };

export interface Metadata {
    identifier: Identifier;
    version: Version;
    dependencies: Record<Identifier, Version>;
}

export interface Registrar {
    registerRenderObject<PS extends ParamBase<symbol, unknown>[]>(
        renderObject: RenderObjectDef<PS>,
    ): void;
    registerFileHandle(fileHandle: FileHandleDef): void;
    registerShader(shader: ShaderDef): void;
    registerSpeaker(speaker: SpeakerDef): void;
}

export interface PluginExports {
    metadata(): Metadata;
    register?(registrar: Registrar): void;
    load?(): void;
    unload?(): void;
    readCompute?(identifier: Identifier): Param;
    writeCompute?(identifier: Identifier, written: Param): void;
    fileHandle?(handler: FileHandle): Param;
    speaker?(identifier: Identifier): AudioBuffer;
    window?(): JSX.Element;
}

export const extractSource = async (source: string): Promise<PluginExports> => {
    const module = await import(source);
    throw new Error("todo");
};
