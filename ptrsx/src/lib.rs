// #[cfg(not(target_os = "macos"))]
// pub mod c32;
// #[cfg(not(target_os = "macos"))]
// pub mod d32;
// pub mod s32;

pub mod c64;
pub mod d64;
pub mod s64;

pub mod sc64;

pub mod error;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const DEFAULT_BUF_SIZE: usize = 0x4000;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub const DEFAULT_BUF_SIZE: usize = 0x100000;

#[cfg(any(
    all(target_os = "windows", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "x86_64"),
))]
pub const DEFAULT_BUF_SIZE: usize = 0x1000;

pub const PTRHEADER64: [u8; 8] = [b'P', b'T', b'R', 64, 0, 0, 0, 0];

pub const PTRHEADER32: [u8; 8] = [b'P', b'T', b'R', 32, 0, 0, 0, 0];
