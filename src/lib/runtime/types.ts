/* eslint-disable */
// ============================================= //
// Types for WebAssembly runtime                 //
//                                               //
// This file is generated. PLEASE DO NOT MODIFY. //
// ============================================= //

export type ComponentType =
    | { type: "I8" }
    | { type: "I16" }
    | { type: "U8" }
    | { type: "U16" }
    | { type: "F16" }
    | { type: "F32" };

export type FileFormat = {
    has_audio: boolean;
    has_video: boolean;
    codec: number;
};

export type Metadata = {
    identifier: string;
    version: VersionNumber;
    dependencies: Array<string>;
};

export type RenderObject = {
    inputs: Array<ShaderInput>;
    vertex_shader: Shader;
    fragment_shader: Shader;
};

export type RenderObjectFactoryError = "Unsupported" | "OutOfMemory" | { IllegalFormat: string };

/**
 * A result that can be either successful (`Ok`) or represent an error (`Err`).
 */
export type Result<T, E> =
    /**
     * Represents a successful result.
     */
    | { Ok: T }
    /**
     * Represents an error.
     */
    | { Err: E };

export type Shader = {
    identifier: string;
    source: string;
};

export type ShaderInput =
    | {
          type: "In";
          payload: {
              key_path: string;
              location: number;
              components_per_vertex: number;
              component_type: ComponentType;
              normalized?: boolean;
              strides?: number;
              offset?: number;
          };
      }
    | { type: "Uniform"; payload: { key_path: string; uniform_type: UniformType } };

export type UniformType =
    | { type: "Int" }
    | { type: "UInt" }
    | { type: "Float" }
    | { type: "FloatMatrix"; payload: { columns: number; rows: number } };

export type VersionNumber = {
    major: number;
    minor: number;
    patch: number;
};
