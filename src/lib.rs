//! A crate solely housing [`Buffer`] and [`ConstBuffer`]
#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]

#[cfg(feature = "std")]
mod buffer;
#[cfg(feature = "std")]
pub use buffer::Buffer;

mod const_buffer;
pub use const_buffer::ConstBuffer;

use crate::traits::{BufferMetrics, BufferPointers};

/// Simple, essential, traits buffers must implement
pub mod traits;

mod buffer_compatibility;

// pub mod formats;

/// Draw a pixel color onto the buffer without checking if the pixel is on screen (which will crash the program if it isn't)
#[inline(always)]
#[allow(clippy::inline_always)]
#[track_caller]
pub const fn draw_pixel_unsafe(
    buffer: &mut (impl [const] BufferPointers + [const] BufferMetrics),
    xy: (usize, usize),
    color: u32,
) {
    unsafe {
        *buffer.mut_pointer().add(xy.1 * buffer.width() + xy.0) = color;
    }
}
/// Draw a pixel color onto the buffer by first checking if the pixel is on screen
#[inline(always)]
#[allow(clippy::inline_always)]
#[track_caller]
pub const fn draw_pixel_safe(
    buffer: &mut (impl [const] BufferPointers + [const] BufferMetrics),
    xy: (usize, usize),
    color: u32,
) {
    if xy.0 < buffer.width() && xy.1 < buffer.height() {
        unsafe {
            *buffer.mut_pointer().add(xy.1 * buffer.width() + xy.0) = color;
        }
    }
}

/// Commonly used components
pub mod prelude;

// TODO: Use the code below
// /// Create a buffer from a bitmask sequence
// ///
// /// # Errors
// /// When the given dimensions don't match
// pub fn buffer_from_bitmask(
//     mask: &[bool],
//     size: (usize, usize),
//     color_0: u32,
//     color_1: u32,
// ) -> Result<Buffer, String> {
//     let mut new = Vec::with_capacity(mask.len());
//     for i in mask {
//         if *i {
//             new.push(color_1);
//         } else {
//             new.push(color_0);
//         }
//     }
//     Buffer::new(size, new)
// }

// /// A thread safe double buffer
// #[derive(Debug)]
// #[cfg(feature = "std")]
// #[cfg_attr(feature = "c_compatible", repr(C))]
// pub struct DoubleBuffer {
//     front: Buffer,
//     back: Buffer,
//     front_is_back: std::sync::atomic::AtomicBool, // true if front buffer is the "back" buffer
// }

// #[cfg(feature = "std")]
// impl DoubleBuffer {
//     /// Creates a new instance of the double buffer starting out empty
//     #[must_use]
//     pub fn new(width: usize, height: usize) -> Self {
//         Self {
//             front: Buffer::new_empty((width, height)),
//             back: Buffer::new_empty((width, height)),
//             front_is_back: std::sync::atomic::AtomicBool::new(false),
//         }
//     }

//     /// Renderer reads from the front buffer
//     pub fn read(&self) -> &Buffer {
//         if self
//             .front_is_back
//             .load(std::sync::atomic::Ordering::Acquire)
//         {
//             &self.back
//         } else {
//             &self.front
//         }
//     }

//     /// Sim writes to the back buffer, then swaps
//     pub fn write(&mut self, new_data: Buffer) {
//         if self
//             .front_is_back
//             .load(std::sync::atomic::Ordering::Acquire)
//         {
//             self.front = new_data;
//             self.front_is_back
//                 .store(false, std::sync::atomic::Ordering::Release);
//         } else {
//             self.back = new_data;
//             self.front_is_back
//                 .store(true, std::sync::atomic::Ordering::Release);
//         }
//     }
// }
