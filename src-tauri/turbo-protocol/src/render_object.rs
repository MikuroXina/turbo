use std::collections::HashMap;

use fp_bindgen::prelude::*;
use thiserror::Error;

use self::param::Param;
use crate::common::Identifier;

pub mod param;
pub mod shader;

#[repr(u8)]
#[non_exhaustive]
#[derive(Error, Debug, Clone, PartialEq, Eq, Hash, Serializable)]
pub enum RenderObjectFactoryError {
    #[error("unsupported format")]
    Unsupported,
    #[error("out of memory")]
    OutOfMemory,
    #[error("illegal format: {0}")]
    IllegalFormat(String),
}

#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serializable)]
pub struct RenderObject {
    pub identifier: Identifier,
    pub name: String,
    pub parameter: HashMap<String, Param>,
}
