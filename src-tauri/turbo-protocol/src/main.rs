use bytes::Bytes;
use fp_bindgen::{prelude::*, types::CargoDependency};
use std::collections::{BTreeMap, BTreeSet};

use self::{
    common::{Identifier, KeyPath, Metadata},
    render_object::{
        param::{Param, PrimitiveParam},
        shader::Shader,
        RenderObject,
    },
};

pub mod common;
pub mod render_object;

fp_import! {
    fn log(message: String);

    fn get_from_store(key: KeyPath) -> Option<PrimitiveParam>;

    fn register_render_object(obj: RenderObject);
    fn register_shader(obj: Shader);
}

fp_export! {
    fn metadata() -> Metadata;

    fn load();
    fn unload();

    fn on_compute_parameter(identifier: Identifier) -> Param;
    fn on_file_handle(identifier: Identifier, bytes: Bytes) -> Param;
}

const VERSION: &str = "0.1.0";
const AUTHORS: &str = r#"["MikuroXina <ryosukadnak@gmail.com>"]"#;
const NAME: &str = "turbo-plugin";

fn plugin_dependencies() -> BTreeMap<&'static str, CargoDependency> {
    BTreeMap::from([
        (
            "fp-bindgen-support",
            CargoDependency {
                version: Some("2.2.0"),
                features: BTreeSet::from(["async", "guest"]),
                ..Default::default()
            },
        ),
        (
            "bytes",
            CargoDependency {
                version: Some("1.2.1"),
                ..Default::default()
            },
        ),
    ])
}

fn main() {
    for cfg in [
        BindingConfig {
            bindings_type: BindingsType::RustPlugin(RustPluginConfig {
                name: NAME,
                authors: AUTHORS,
                version: VERSION,
                dependencies: plugin_dependencies(),
            }),
            path: "../turbo-plugin",
        },
        BindingConfig {
            bindings_type: BindingsType::TsRuntimeWithExtendedConfig(
                TsExtendedRuntimeConfig::new().with_raw_export_wrappers(),
            ),
            path: "../../src/lib/runtime",
        },
    ] {
        println!("Generating bindings to `{}/`", cfg.path);
        fp_bindgen!(cfg);
    }
}
