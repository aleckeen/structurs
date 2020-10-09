use std::io;

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
///   let val = c.read_as::<Test>().unwrap();
///   assert_eq!(
///     Test {
///       first_field: -15,
///       array: [25, 97]
///     },
///     val
///   );
/// }
/// ```
pub trait Reader: io::Read
{
  #[inline]
  fn read_as<T>(&mut self) -> io::Result<T>
  where
    T: Read,
    Self: Sized,
  {
    T::read(self)
  }

  #[inline]
  fn read_le<T>(&mut self) -> io::Result<T>
  where
    T: PrimitiveRead,
    Self: Sized,
  {
    T::read_le(self)
  }

  #[inline]
  fn read_be<T>(&mut self) -> io::Result<T>
  where
    T: PrimitiveRead,
    Self: Sized,
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
