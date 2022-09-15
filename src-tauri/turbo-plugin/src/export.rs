use crate::types::*;

#[fp_bindgen_support::fp_export_signature]
pub fn load();

#[fp_bindgen_support::fp_export_signature]
pub fn metadata() -> Metadata;

#[fp_bindgen_support::fp_export_signature]
pub fn on_compute_parameter(identifier: Identifier) -> Param;

#[fp_bindgen_support::fp_export_signature]
pub fn on_file_handle(identifier: Identifier, bytes: bytes::Bytes) -> Param;

#[fp_bindgen_support::fp_export_signature]
pub fn unload();
