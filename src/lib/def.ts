import type { Anchor } from "./anchor";
import type { Identifier } from "./exports";
import type { Size } from "./geo";

export interface ParamBase<S, T> {
    type: S;
    defaultVale: T;
}

export const booleanParam = Symbol("BOOLEAN");
export interface BooleanParam extends ParamBase<typeof booleanParam, boolean> {
    radioButtonGroup?: string;
}

export const intParam = Symbol("INT");
export interface IntParam extends ParamBase<typeof intParam, number> {
    minValue?: number;
    maxValue?: number;
    steps?: number;
    useSlider?: boolean;
}

export const floatParam = Symbol("FLOAT");
export interface FloatParam extends ParamBase<typeof floatParam, number> {
    minValue?: number;
    maxValue?: number;
    steps?: number;
    useSlider?: boolean;
}

export const shortTextParam = Symbol("SHORT_TEXT");
export interface ShortTextParam extends ParamBase<typeof shortTextParam, string> {
    placeholderText: string;
    maxLength?: number;
}

export const longTextParam = Symbol("LONG_TEXT");
export interface LongTextParam extends ParamBase<typeof longTextParam, string> {
    placeholderText: string;
    maxColumns?: number;
    maxRows?: number;
}

export const matParam = Symbol("MAT");
export interface MatParam extends ParamBase<typeof matParam, readonly number[]> {
    shape: readonly number[];
    componentNames: readonly string[];
    minValues?: readonly number[];
    maxValues?: readonly number[];
}

export const anchorParam = Symbol("ANCHOR");
export type AnchorParam = ParamBase<typeof anchorParam, Readonly<Anchor>>;

export const textureParam = Symbol("TEXTURE");
export interface TextureParam extends ParamBase<typeof textureParam, Identifier> {
    minSize: Size;
    maxSize: Size;
}

export const audioSampleParam = Symbol("AUDIO_SAMPLE");
export interface AudioSampleParam extends ParamBase<typeof audioSampleParam, Identifier> {
    sampleRate?: number;
    channels: number;
}

export type FileHandleOutput =
    | typeof booleanParam
    | typeof intParam
    | typeof floatParam
    | typeof shortTextParam
    | typeof matParam
    | typeof textureParam
    | typeof audioSampleParam;
export const fileHandleParam = Symbol("FILE_HANDLE");
export interface FileHandleParam extends ParamBase<typeof fileHandleParam, Identifier> {
    outputType: FileHandleOutput;
}

export const shaderParam = Symbol("SHADER");
export type ShaderParam = ParamBase<typeof shaderParam, Identifier>;

export const speakerParam = Symbol("SPEAKER");
export type SpeakerParam = ParamBase<typeof speakerParam, Identifier>;

export const renderObjectRefParam = Symbol("RENDER_OBJECT_REF");
export type RenderObjectRefParam = ParamBase<typeof renderObjectRefParam, Identifier>;

export interface RenderObjectDef<PS extends ParamBase<symbol, unknown>[]> {
    identifier: Identifier;
    params: PS;
}

export interface FileHandleDef {
    identifier: Identifier;
}

export interface ShaderDef {
    identifier: Identifier;
}

export interface SpeakerDef {
    identifier: Identifier;
}
