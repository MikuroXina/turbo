use fp_bindgen::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
pub struct Shader {
    pub identifier: String,
    pub source: String,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
#[fp(tag = "type", content = "payload")]
pub enum ShaderInput {
    In {
        key_path: String,
        location: u32,
        components_per_vertex: u32,
        component_type: ComponentType,
        normalized: Option<bool>,
        strides: Option<u32>,
        offset: Option<u32>,
    },
    Uniform {
        key_path: String,
        uniform_type: UniformType,
    },
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
#[fp(tag = "type")]
pub enum ComponentType {
    I8,
    I16,
    U8,
    U16,
    F16,
    F32,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
#[fp(tag = "type", content = "payload")]
pub enum UniformType {
    Int,
    UInt,
    Float,
    FloatMatrix { columns: u8, rows: u8 },
}
