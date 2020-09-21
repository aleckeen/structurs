//! # Structurs
//!
//! Structurs is a framework that makes reading bytes from a source easier. It has traits and
//! derive macros that are used to read bytes into a data type.

use std::io;

pub use structurs_derive::*;

/// This trait can be used to read all kinds of data types that implement [`structurs::Read`] or
/// ['structurs::PrimitiveRead'] from a source.
///
/// ```
/// use std::io::Cursor;
/// use structurs::Reader;
///
/// #[derive(structurs::Read, Debug, Eq, PartialEq)]
/// struct Test
/// {
///   first_field: i32,
///   array: [u16; 2],
/// }
///
/// fn main()
/// {
///   let mut c: Cursor<Vec<u8>> = Cursor::new(vec![241, 255, 255, 255, 25, 0, 97, 0]);
///   let val = c.read::<Test>().unwrap();
///   assert_eq!(
///     Test {
///       first_field: -15,
///       array: [25, 97]
///     },
///     val
///   );
/// }
/// ```
pub trait Reader
{
  #[inline]
  fn read<T>(&mut self) -> io::Result<T>
  where
    T: Read,
    Self: Sized + io::Read,
  {
    T::read(self)
  }

  #[inline]
  fn read_le<T>(&mut self) -> io::Result<T>
  where
    T: PrimitiveRead,
    Self: Sized + io::Read,
  {
    T::read_le(self)
  }

  #[inline]
  fn read_be<T>(&mut self) -> io::Result<T>
  where
    T: PrimitiveRead,
    Self: Sized + io::Read,
  {
    T::read_be(self)
  }
}

impl<T> Reader for T where T: io::Read {}

/// This trait can be used to read data types that can be represented in either big-endian or
/// little-endian format like [`u64`].
pub trait PrimitiveRead
{
  /// Reads a primitive type from a source in little-edian format.
  /// ```
  /// use std::io::Cursor;
  /// use structurs::PrimitiveRead;
  ///
  /// fn main()
  /// {
  ///   let mut c: Cursor<Vec<u8>> = Cursor::new(vec![87, 0, 0, 0]);
  ///   let val = u32::read_le(&mut c).unwrap();
  ///   assert_eq!(87, val);
  /// }
  /// ```
  fn read_le<R>(reader: &mut R) -> io::Result<Self>
  where
    R: io::Read,
    Self: Sized;

  /// Reads a primitive type from a source in big-edian format.
  /// ```
  /// use std::io::Cursor;
  /// use structurs::PrimitiveRead;
  ///
  /// fn main()
  /// {
  ///   let mut c: Cursor<Vec<u8>> = Cursor::new(vec![0, 0, 0, 226]);
  ///   let val = u32::read_be(&mut c).unwrap();
  ///   assert_eq!(226, val);
  /// }
  /// ```
  fn read_be<R>(reader: &mut R) -> io::Result<Self>
  where
    R: io::Read,
    Self: Sized;

  /// Reads a primitive type from a source in the native format that the CPU uses. Most of the CPU
  /// architectures use little-endian format.
  #[cfg(target_endian = "little")]
  #[inline]
  fn read_ne<R>(reader: &mut R) -> io::Result<Self>
  where
    R: io::Read,
    Self: Sized,
  {
    Self::read_le(reader)
  }

  #[cfg(target_endian = "big")]
  #[inline]
  fn read_ne<R>(reader: &mut R) -> io::Result<Self>
  where
    R: io::Read,
    Self: Sized,
  {
    Self::read_be(reader)
  }
}

/// This trait can be used to read data structures that are composed of other fields that implement
/// [`structurs::Read`] or [`structurs::PrimitiveRead`]. See [`structurs_derive`].
pub trait Read
{
  fn read<R>(reader: &mut R) -> io::Result<Self>
  where
    R: io::Read,
    Self: Sized;
}

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
