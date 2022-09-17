/* eslint-disable */
// ============================================= //
// Types for WebAssembly runtime                 //
//                                               //
// This file is generated. PLEASE DO NOT MODIFY. //
// ============================================= //

export type Anchor = {
    vertical: VerticalAnchor;
    horizontal: HorizontalAnchor;
};

export type AudioSample = {
    bytes: Uint8Array;
    sample_rate: number;
    channels: number;
};

export type ComponentType = "I8" | "I16" | "U8" | "U16" | "F16" | "F32";

export type ComputedParam = {
    identifier: Identifier;
};

export type Dependency = {
    plugin_id: Identifier;
    version: VersionNumber;
};

export type FileHandle = {
    identifier: Identifier;
    output_type: FileHandleOutput;
};

export type FileHandleOutput =
    | "Boolean"
    | "Int"
    | "Float"
    | "ShortText"
    | "LongText"
    | "Vec2"
    | "Vec3"
    | "Mat2x2"
    | "Mat2x3"
    | "Mat2x4"
    | "Mat3x2"
    | "Mat3x3"
    | "Mat3x4"
    | "Mat4x2"
    | "Mat4x3"
    | "Mat4x4"
    | "Anchor"
    | "Texture"
    | "AudioSample"
    | "Shader"
    | "Speaker";

export type HorizontalAnchor = "Left" | "Center" | "Right";

export type Identifier = string;

export type KeyPath = Array<string>;

export type Mat2x2 = {
    components: Float64Array;
};

export type Mat2x3 = {
    components: Float64Array;
};

export type Mat2x4 = {
    components: Float64Array;
};

export type Mat3x2 = {
    components: Float64Array;
};

export type Mat3x3 = {
    components: Float64Array;
};

export type Mat3x4 = {
    components: Float64Array;
};

export type Mat4x2 = {
    components: Float64Array;
};

export type Mat4x3 = {
    components: Float64Array;
};

export type Mat4x4 = {
    components: Float64Array;
};

export type Metadata = {
    plugin_id: Identifier;
    version: VersionNumber;
    dependencies: Array<Dependency>;
};

export type Param = { Primitive: PrimitiveParam } | { Computed: ComputedParam };

export type PrimitiveParam =
    | { Boolean: boolean }
    | { Int: number }
    | { Float: number }
    | { ShortText: string }
    | { LongText: string }
    | { Vec2: Vec2 }
    | { Vec3: Vec3 }
    | { Vec4: Vec4 }
    | { Mat2x2: Mat2x2 }
    | { Mat2x3: Mat2x3 }
    | { Mat2x4: Mat2x4 }
    | { Mat3x2: Mat3x2 }
    | { Mat3x3: Mat3x3 }
    | { Mat3x4: Mat3x4 }
    | { Mat4x2: Mat4x2 }
    | { Mat4x3: Mat4x3 }
    | { Mat4x4: Mat4x4 }
    | { Anchor: Anchor }
    | { Texture: Texture }
    | { AudioSample: AudioSample }
    | { FileHandle: FileHandle }
    | { Shader: Shader }
    | { Speaker: Speaker };

export type RenderObject = {
    identifier: Identifier;
    name: string;
    parameter: Record<string, Param>;
};

export type Shader = {
    identifier: Identifier;
    source: string;
    inputs: Array<ShaderInput>;
};

export type ShaderInput =
    | {
          In: {
              path: KeyPath;
              location: number;
              components_per_vertex: number;
              component_type: ComponentType;
              normalized?: boolean;
              strides?: number;
              offset?: number;
          };
      }
    | { Uniform: { path: KeyPath; uniform_type: UniformType } };

export type Speaker = {
    identifier: Identifier;
};

export type Texture = {
    bytes: Uint8Array;
    width: number;
    height: number;
};

export type UniformType =
    | "Int"
    | "UInt"
    | "Float"
    | "Mat2x2"
    | "Mat2x3"
    | "Mat2x4"
    | "Mat3x2"
    | "Mat3x3"
    | "Mat3x4"
    | "Mat4x2"
    | "Mat4x3"
    | "Mat4x4";

export type Vec2 = {
    components: Float64Array;
};

export type Vec3 = {
    components: Float64Array;
};

export type Vec4 = {
    components: Float64Array;
};

export type VersionNumber = {
    major: number;
    minor: number;
    patch: number;
};

export type VerticalAnchor = "Top" | "Center" | "Bottom";
