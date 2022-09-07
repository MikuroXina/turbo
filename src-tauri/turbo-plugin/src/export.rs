use crate::types::*;

#[fp_bindgen_support::fp_export_signature]
pub fn load();

#[fp_bindgen_support::fp_export_signature]
pub fn metadata() -> Metadata;

#[fp_bindgen_support::fp_export_signature]
pub fn render_object_factory(format: FileFormat, data: bytes::Bytes) -> Result<RenderObject, RenderObjectFactoryError>;

#[fp_bindgen_support::fp_export_signature]
pub fn unload();
