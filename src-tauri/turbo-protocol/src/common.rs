use fp_bindgen::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serializable)]
pub struct Identifier(pub String);

impl<S> From<S> for Identifier
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serializable)]
pub struct KeyPath(Vec<String>);

impl std::fmt::Display for KeyPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("."))
    }
}

impl KeyPath {
    pub fn new<P>(path: P) -> Self
    where
        P: IntoIterator,
        <P as IntoIterator>::Item: Into<String>,
    {
        Self(path.into_iter().map(Into::into).collect())
    }
}

impl<P> From<P> for KeyPath
where
    P: IntoIterator,
    <P as IntoIterator>::Item: Into<String>,
{
    fn from(path: P) -> Self {
        Self::new(path)
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serializable)]
pub struct Metadata {
    pub plugin_id: Identifier,
    pub version: VersionNumber,
    pub dependencies: Vec<Dependency>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serializable)]
pub struct Dependency {
    pub plugin_id: Identifier,
    pub version: VersionNumber,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serializable)]
pub struct VersionNumber {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}
