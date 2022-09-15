use crate::types::*;

#[fp_bindgen_support::fp_import_signature]
pub fn get_from_store(key: KeyPath) -> Option<PrimitiveParam>;

#[fp_bindgen_support::fp_import_signature]
pub fn log(message: String);

#[fp_bindgen_support::fp_import_signature]
pub fn register_render_object(obj: RenderObject);

#[fp_bindgen_support::fp_import_signature]
pub fn register_shader(obj: Shader);
