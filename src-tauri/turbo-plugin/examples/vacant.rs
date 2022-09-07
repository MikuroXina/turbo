use std::panic;

use turbo_plugin::{
    fp_export_impl, log, FileFormat, Metadata, RenderObject, RenderObjectFactoryError,
    VersionNumber,
};

fn init_panic_hook() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(|info| log(info.to_string())));
    });
}

#[fp_export_impl(turbo_plugin)]
pub fn load() {
    init_panic_hook();
}

#[fp_export_impl(turbo_plugin)]
pub fn metadata() -> Metadata {
    Metadata {
        identifier: "com.mikuroxina.vacant".into(),
        version: VersionNumber {
            major: 0,
            minor: 1,
            patch: 0,
        },
        dependencies: vec![],
    }
}

#[fp_export_impl(turbo_plugin)]
pub fn render_object_factory(
    _format: FileFormat,
    _data: bytes::Bytes,
) -> Result<RenderObject, RenderObjectFactoryError> {
    Err(RenderObjectFactoryError::Unsupported)
}

#[fp_export_impl(turbo_plugin)]
pub fn unload() {}
