//! GPU buffer writing utilities

use std::num::NonZeroU64;

use rice_dom::{Color, Style};
use rice_layout::Rect;
use wgpu::*;

/// Trait for data structures that can be written to a GPU buffer
pub trait WriteBuffer {
    /// Size of the data structure in bytes
    const SIZE: usize;

    /// Write bytes to a buffer
    fn write_buffer(&self, buffer: &mut [u8]);
}

impl WriteBuffer for Rect {
    const SIZE: usize = std::mem::size_of::<[i32; 4]>();

    fn write_buffer(&self, buffer: &mut [u8]) {
        buffer[..Self::SIZE].copy_from_slice(bytemuck::bytes_of(&[
            self.size[0],
            self.size[1],
            self.position[0],
            self.position[1],
        ]));
    }
}

impl WriteBuffer for Color {
    const SIZE: usize = std::mem::size_of::<[f32; 4]>();

    fn write_buffer(&self, buffer: &mut [u8]) {
        buffer[..Self::SIZE].copy_from_slice(bytemuck::bytes_of(&[self.r, self.g, self.b, self.a]));
    }
}

impl WriteBuffer for Style {
    const SIZE: usize = Color::SIZE;

    fn write_buffer(&self, buffer: &mut [u8]) {
        self.background_color
            .write_buffer(&mut buffer[..Color::SIZE]);
    }
}

/// Write a slice of data to a GPU buffer, indexed by a list of indices
pub fn write_indexed_slice_to_buffer<T: WriteBuffer>(
    data: &[T],
    indices: &[usize],
    buffer: &Buffer,
    queue: &Queue,
) {
    let size = T::SIZE * indices.len();
    let size = NonZeroU64::new(size as u64).unwrap();

    let mut buffer = queue
        .write_buffer_with(buffer, 0, size)
        .expect("Failed to map buffer for writing");

    for &i in indices {
        data[i].write_buffer(&mut buffer[i * T::SIZE..(i + 1) * T::SIZE]);
    }
}
