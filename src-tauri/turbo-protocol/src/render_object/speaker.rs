use fp_bindgen::prelude::*;

use crate::common::Identifier;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
pub struct Speaker {
    pub identifier: Identifier,
}
