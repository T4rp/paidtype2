#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg_attr(target_os = "windows", path = "win_freetype.rs")]
pub mod freetype;

#[cfg(target_os = "linux")]
pub mod freetype;

