//! # Structurs
//!
//! Structurs is a framework that makes reading bytes from a source easier. It has traits and
//! derive macros that are used to read bytes into a data type.
//!
//! # Macro
//!
//! This crate provides a macro that lets you automatically implement [`structurs::Read`] trait for
//! you. You need to enable `derive` feature to use this macro.
//!
//! ```edition2018, ignore
//! #[derive(structurs::Read)]
//! ```
//!
//! Writing a function that reads bytes into a structure can be a tedious job.
//! `#[derive(structurs::Read)]` macro automatically generates code that implements this trait for
//! your structure. It also includes attributes that can change the way some of the fields are
//! read.
//!
//! By default all fields will be read using [`structurs::Read::read`] function, but you might have
//! fields that might need to be read in big-endian format. In that case you can mark those fields
//! with `#[be]` attribute.
//!
//! ## Attributes
//!
//! The following is the list of attributes that can be used to mark the fields of structures.
//!
//! - `#[le]`, This denotes that the field is in little-endian format.
//! - `#[be]`, This denotes that the field is in big-endian format.
//! - `#[ne]`, This denotes that the field is in CPU's native endian format. Most CPU's will use
//!   little-endian format.
//! - `#[pad]`, This denotes that the field is a padding and is not important. In this case the
//!   field will be initialized to its default value using [`Default::default`]. By default the
//!   length of the field type worth of bytes will be read from the reader. You can also pass a
//!   `bytes` value to this attribute. `#[pad(bytes = N)]` means that N bytes should be read from
//!   the reader in which case field type is not important and should be [`structurs::Pad`].
//!
//! ## Example
//!
//! ```
//! #[derive(structurs::Read)]
//! struct Test
//! {
//!   // This field will be read using structurs::Read::read function.
//!   field_1: i64,
//!   // This field will be read using structurs::PrimitiveRead::read_le function.
//!   #[le]
//!   field_2: i16,
//!   // This field will not be read. But 8 bytes will be read from the reader and discarded.
//!   #[pad(bytes = 8)]
//!   pad_field: structurs::Pad,
//! }
//! ```
//!
//! ## Note
//!
//! This macro currently only supports structs with named fields.

use std::io;

mod read;
mod write;

pub use read::{PrimitiveRead, Read, Reader};
pub use write::{PrimitiveWrite, Write, Writer};

#[cfg(feature = "derive")]
pub use structurs_derive::*;

/// Data structure for padding fields.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct Pad;

macro_rules! impl_primitive {
  ($ty:ty, $bytes:expr) => {
    impl PrimitiveRead for $ty
    {
      #[inline]
      fn read_le<R>(reader: &mut R) -> io::Result<Self>
      where
        R: io::Read,
      {
        let mut buf: [u8; $bytes] = [0; $bytes];
        reader.read_exact(&mut buf)?;
        Ok(<$ty>::from_le_bytes(buf))
      }

      #[inline]
      fn read_be<R>(reader: &mut R) -> io::Result<Self>
      where
        R: io::Read,
      {
        let mut buf: [u8; $bytes] = [0; $bytes];
        reader.read_exact(&mut buf)?;
        Ok(<$ty>::from_be_bytes(buf))
      }
    }

    impl PrimitiveWrite for $ty
    {
      #[inline]
      fn write_le<W>(&self, writer: &mut W) -> io::Result<()>
      where
        W: io::Write,
      {
        writer.write_all(&self.to_le_bytes())
      }

      #[inline]
      fn write_be<W>(&self, writer: &mut W) -> io::Result<()>
      where
        W: io::Write,
      {
        writer.write_all(&self.to_be_bytes())
      }
    }

    impl Read for $ty
    {
      #[inline]
      fn read<R>(reader: &mut R) -> io::Result<Self>
      where
        R: io::Read,
        Self: PrimitiveRead,
      {
        Self::read_ne(reader)
      }
    }

    impl Write for $ty
    {
      #[inline]
      fn write<W>(&self, writer: &mut W) -> io::Result<()>
      where
        W: io::Write,
        Self: PrimitiveWrite,
      {
        self.write_ne(writer)
      }
    }
  };
}

impl_primitive!(u8, 1);
impl_primitive!(u16, 2);
impl_primitive!(u32, 4);
impl_primitive!(u64, 8);
impl_primitive!(u128, 16);
impl_primitive!(i8, 1);
impl_primitive!(i16, 2);
impl_primitive!(i32, 4);
impl_primitive!(i64, 8);
impl_primitive!(i128, 16);
impl_primitive!(f32, 4);
impl_primitive!(f64, 8);
