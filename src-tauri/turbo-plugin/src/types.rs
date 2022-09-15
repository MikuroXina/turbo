#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct Anchor {
    pub vertical: VerticalAnchor,
    pub horizontal: HorizontalAnchor,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct AudioSample {
    pub bytes: bytes::Bytes,
    pub sample_rate: u32,
    pub channels: u8,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum ComponentType {
    I8,
    I16,
    U8,
    U16,
    F16,
    F32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct ComputedParam {
    pub identifier: Identifier,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct Dependency {
    pub plugin_id: Identifier,
    pub version: VersionNumber,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct FileHandle {
    pub identifier: Identifier,
    pub output_type: FileHandleOutput,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum FileHandleOutput {
    Boolean,
    Int,
    Float,
    ShortText,
    LongText,
    Vec2,
    Vec3,
    Mat2x2,
    Mat2x3,
    Mat2x4,
    Mat3x2,
    Mat3x3,
    Mat3x4,
    Mat4x2,
    Mat4x3,
    Mat4x4,
    Anchor,
    Texture,
    AudioSample,
    Shader,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum HorizontalAnchor {
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct Identifier(pub String);

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct KeyPath(pub Vec<String>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mat2x2 {
    pub components: [f64; 4],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mat2x3 {
    pub components: [f64; 6],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mat2x4 {
    pub components: [f64; 8],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mat3x2 {
    pub components: [f64; 6],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mat3x3 {
    pub components: [f64; 9],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mat3x4 {
    pub components: [f64; 12],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mat4x2 {
    pub components: [f64; 8],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mat4x3 {
    pub components: [f64; 12],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mat4x4 {
    pub components: [f64; 16],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct Metadata {
    pub plugin_id: Identifier,
    pub version: VersionNumber,
    pub dependencies: Vec<Dependency>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Param {
    Primitive(PrimitiveParam),
    Computed(ComputedParam),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PrimitiveParam {
    Boolean(bool),
    Int(u32),
    Float(f64),
    ShortText(String),
    LongText(String),
    Vec2(Vec2),
    Vec3(Vec3),
    Mat2x2(Mat2x2),
    Mat2x3(Mat2x3),
    Mat2x4(Mat2x4),
    Mat3x2(Mat3x2),
    Mat3x3(Mat3x3),
    Mat3x4(Mat3x4),
    Mat4x2(Mat4x2),
    Mat4x3(Mat4x3),
    Mat4x4(Mat4x4),
    Anchor(Anchor),
    Texture(Texture),
    AudioSample(AudioSample),
    FileHandle(FileHandle),
    Shader(Shader),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RenderObject {
    pub identifier: Identifier,
    pub name: String,
    pub parameter: HashMap<KeyPath, Param>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct Shader {
    pub identifier: Identifier,
    pub source: String,
    pub inputs: Vec<ShaderInput>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum ShaderInput {
    In {
        path: KeyPath,
        location: u32,
        components_per_vertex: u32,
        component_type: ComponentType,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        normalized: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        strides: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        offset: Option<u32>,
    },
    Uniform {
        path: KeyPath,
        uniform_type: UniformType,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct Texture {
    pub bytes: bytes::Bytes,
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum UniformType {
    Int,
    UInt,
    Float,
    Mat2x2,
    Mat2x3,
    Mat2x4,
    Mat3x2,
    Mat3x3,
    Mat3x4,
    Mat4x2,
    Mat4x3,
    Mat4x4,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Vec2 {
    pub components: [f64; 2],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Vec3 {
    pub components: [f64; 2],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct VersionNumber {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum VerticalAnchor {
    Top,
    Center,
    Bottom,
}
