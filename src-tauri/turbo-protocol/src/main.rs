use bytes::Bytes;
use fp_bindgen::{prelude::*, types::CargoDependency};
use once_cell::sync::Lazy;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

use self::shader::{Shader, ShaderInput};

mod shader;

fp_import! {
    fn log(message: String);
}

fp_export! {
    fn metadata() -> Metadata;

    fn load();
    fn unload();

    fn render_object_factory(format: FileFormat, data: Bytes) -> Result<RenderObject, RenderObjectFactoryError>;
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serializable)]
pub struct Metadata {
    pub identifier: String,
    pub version: VersionNumber,
    pub dependencies: Vec<String>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serializable)]
pub struct VersionNumber {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
pub struct FileFormat {
    pub has_audio: bool,
    pub has_video: bool,
    pub codec: u64,
}

#[repr(u8)]
#[derive(Error, Debug, Clone, PartialEq, Eq, Serializable)]
pub enum RenderObjectFactoryError {
    #[error("unsupported format")]
    Unsupported,
    #[error("out of memory")]
    OutOfMemory,
    #[error("illegal format: {0}")]
    IllegalFormat(String),
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
pub struct RenderObject {
    pub inputs: Vec<ShaderInput>,
    pub vertex_shader: Shader,
    pub fragment_shader: Shader,
}

const VERSION: &str = "0.1.0";
const AUTHORS: &str = r#"["MikuroXina <ryosukadnak@gmail.com>"]"#;
const NAME: &str = "turbo-plugin";

static PLUGIN_DEPENDENCIES: Lazy<BTreeMap<&str, CargoDependency>> = Lazy::new(|| {
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
});

fn main() {
    for cfg in [
        BindingConfig {
            bindings_type: BindingsType::RustPlugin(RustPluginConfig {
                name: NAME,
                authors: AUTHORS,
                version: VERSION,
                dependencies: PLUGIN_DEPENDENCIES.clone(),
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
