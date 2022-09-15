use fp_bindgen::prelude::*;

use crate::common::{Identifier, KeyPath};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
pub struct Shader {
    pub identifier: Identifier,
    pub source: String,
    pub inputs: Vec<ShaderInput>,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
pub enum ShaderInput {
    In {
        path: KeyPath,
        location: u32,
        components_per_vertex: u32,
        component_type: ComponentType,
        normalized: Option<bool>,
        strides: Option<u32>,
        offset: Option<u32>,
    },
    Uniform {
        path: KeyPath,
        uniform_type: UniformType,
    },
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
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
