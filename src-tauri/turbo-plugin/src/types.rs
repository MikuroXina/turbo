#![allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum ComponentType {
    I8,
    I16,
    U8,
    U16,
    F16,
    F32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct FileFormat {
    pub has_audio: bool,
    pub has_video: bool,
    pub codec: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Metadata {
    pub identifier: String,
    pub version: VersionNumber,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct RenderObject {
    pub inputs: Vec<ShaderInput>,
    pub vertex_shader: Shader,
    pub fragment_shader: Shader,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum RenderObjectFactoryError {
    Unsupported,
    OutOfMemory,
    IllegalFormat(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Shader {
    pub identifier: String,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum ShaderInput {
    In {
        key_path: String,
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
        key_path: String,
        uniform_type: UniformType,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum UniformType {
    Int,
    UInt,
    Float,
    FloatMatrix { columns: u8, rows: u8 },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct VersionNumber {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}
