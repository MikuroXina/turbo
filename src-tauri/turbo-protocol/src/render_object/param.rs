use bytes::Bytes;
use fp_bindgen::prelude::*;

use super::shader::Shader;
use crate::common::Identifier;

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serializable)]
pub enum Param {
    Primitive(PrimitiveParam),
    Computed(ComputedParam),
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serializable)]
pub enum PrimitiveParam {
    Boolean(bool),
    Int(u32),
    Float(f64),
    ShortText(String),
    LongText(String),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Mat2x2(Mat2x2),
    Mat2x3(Mat2x3),
    Mat2x4(Mat2x4),
    Mat3x2(Mat3x2),
    Mat3x3(Mat3x3),
    Mat3x4(Mat3x4),
    Mat4x2(Mat4x2),
    Mat4x3(Mat4x3),
    Mat4x4(Mat4x4),
    Anchor(Anchor),
    Texture(Texture),
    AudioSample(AudioSample),
    FileHandle(FileHandle),
    Shader(Shader),
}

#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serializable)]
pub struct ComputedParam {
    pub identifier: Identifier,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serializable)]
pub struct Vec2 {
    pub components: [f64; 2],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serializable)]
pub struct Vec3 {
    pub components: [f64; 3],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serializable)]
pub struct Vec4 {
    pub components: [f64; 4],
}

macro_rules! pairs {
    ($macro:ident; $($items:literal),*) => {
        pairs!(@ $macro; $($items),*; $($items),*);
    };
    (@ $macro:ident; $x:literal; $($ys:literal),*) => {
        $(
            $macro!($x, $ys);
        )*
    };
    (@ $macro:ident; $x:literal, $($xs:literal),*; $($ys:literal),*) => {
        $(
            $macro!($x, $ys);
        )*
        pairs!(@ $macro; $($xs),*; $($ys),*);
    };
}

macro_rules! def_mat {
    ($rows:literal, $cols:literal) => {
        ::paste::paste! {
            #[repr(C)]
            #[derive(Debug, Default, Clone, Copy, PartialEq)]
            pub struct [<Mat $rows x $cols>] {
                pub components: [f64; $cols * $rows],
            }

            impl ::fp_bindgen::prelude::Serializable for [<Mat $rows x $cols>] {
                fn ident() -> ::fp_bindgen::prelude::TypeIdent {
                    ::fp_bindgen::prelude::TypeIdent::from(stringify!([<Mat $rows x $cols>]))
                }
                fn ty() -> ::fp_bindgen::prelude::Type {
                    ::fp_bindgen::prelude::Type::from_item(
                        &format!(
                            concat!("#[repr(C)] pub struct ",
                            stringify!([<Mat $rows x $cols>]),
                            " {{ pub components : [f64 ; {}], }}"),
                            $cols * $rows
                        ),
                    )
                }
                fn collect_types(types: &mut ::fp_bindgen::prelude::TypeMap) {
                    if let ::std::collections::btree_map::Entry::Vacant(entry)
                        = types.entry(Self::ident())
                    {
                        entry.insert(Self::ty());
                        <[f64; $rows * $cols]>::collect_types(types);
                    }
                }
            }

            impl ::std::convert::From<[f64; $cols * $rows]> for [<Mat $rows x $cols>] {
                fn from(components: [f64; $cols * $rows]) -> Self {
                    Self { components }
                }
            }
            impl<'a> ::std::convert::From<&'a [f64; $cols * $rows]> for [<Mat $rows x $cols>] {
                fn from(components: &'a [f64; $cols * $rows]) -> Self {
                    Self { components: components.clone() }
                }
            }

            impl ::std::ops::Index<(usize, usize)> for [<Mat $rows x $cols>] {
                type Output = f64;
                #[inline(always)]
                fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
                    &self.components[row + $rows * col]
                }
            }
            impl ::std::ops::IndexMut<(usize, usize)> for [<Mat $rows x $cols>] {
                #[inline(always)]
                fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
                    &mut self.components[row + $rows * col]
                }
            }

            impl ::std::ops::Add for [<Mat $rows x $cols>] {
                type Output = Self;
                fn add(mut self, other: Self) -> Self {
                    self += other;
                    self
                }
            }
            impl ::std::ops::AddAssign for [<Mat $rows x $cols>] {
                fn add_assign(&mut self, rhs: Self) {
                    for (idx, elem) in self.components.iter_mut().enumerate() {
                        *elem += rhs.components[idx];
                    }
                }
            }
            impl ::std::ops::Sub for [<Mat $rows x $cols>] {
                type Output = Self;
                fn sub(mut self, other: Self) -> Self {
                    self -= other;
                    self
                }
            }
            impl ::std::ops::SubAssign for [<Mat $rows x $cols>] {
                fn sub_assign(&mut self, rhs: Self) {
                    for (idx, elem) in self.components.iter_mut().enumerate() {
                        *elem -= rhs.components[idx];
                    }
                }
            }
        }
    };
}

pairs!(def_mat; 2, 3, 4);

macro_rules! triplets {
    ($macro:ident; $($items:literal),*) => {
        triplets!(@ $macro; $($items),*; $($items),*; $($items),*);
    };
    (@ $macro:ident; $x:literal; $y:literal; $($zs:literal),*) => {
        $(
            $macro!($x, $y, $zs);
        )*
    };
    (@ $macro:ident; $x:literal; $y:literal, $($ys:literal),*; $($zs:literal),*) => {
        $(
            $macro!($x, $y, $zs);
        )*
        triplets!(@ $macro; $x; $($ys),*; $($zs),*);
    };
    (@ $macro:ident; $x:literal, $($xs:literal),*; $y:literal, $($ys:literal),*; $($zs:literal),*) => {
        $(
            $macro!($x, $y, $zs);
        )*
        triplets!(@ $macro; $($xs),*; $y, $($ys),*; $($zs),*);
    };
}

macro_rules! def_mat_mul {
    ($rows:literal, $cols_rows:literal, $cols:literal) => {
        ::paste::paste! {
            impl ::std::ops::Mul<[<Mat $cols_rows x $cols>]> for [<Mat $rows x $cols_rows>] {
                type Output = [<Mat $rows x $cols>];
                fn mul(self, other: [<Mat $cols_rows x $cols>]) -> Self::Output {
                    let mut res = [<Mat $rows x $cols>]::default();
                    for row in 0..$rows {
                        for col in 0..$cols {
                            res[(row, col)] = (0..$cols_rows).map(|k| self[(row, k)] * other[(k, col)]).sum();
                        }
                    }
                    res
                }
            }
        }
    }
}

triplets!(def_mat_mul; 2, 3, 4);

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serializable)]
pub struct Anchor {
    pub vertical: VerticalAnchor,
    pub horizontal: HorizontalAnchor,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serializable)]
pub enum VerticalAnchor {
    Top,
    #[default]
    Center,
    Bottom,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serializable)]
pub enum HorizontalAnchor {
    Left,
    #[default]
    Center,
    Right,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serializable)]
pub struct Texture {
    // up to 4096x4096
    bytes: Bytes,
    width: u16,
    height: u16,
}

impl Texture {}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serializable)]
pub struct AudioSample {
    // vec of f32
    bytes: Bytes,
    sample_rate: u32,
    channels: u8,
}

impl AudioSample {}

#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serializable)]
pub struct FileHandle {
    pub identifier: Identifier,
    pub output_type: FileHandleOutput,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serializable)]
pub enum FileHandleOutput {
    Boolean,
    Int,
    Float,
    ShortText,
    LongText,
    Vec2,
    Vec3,
    Mat2x2,
    Mat2x3,
    Mat2x4,
    Mat3x2,
    Mat3x3,
    Mat3x4,
    Mat4x2,
    Mat4x3,
    Mat4x4,
    Anchor,
    Texture,
    AudioSample,
    Shader,
}
