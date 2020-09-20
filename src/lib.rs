use std::io;

pub use structurs_derive::*;

pub trait Reader: io::Read
{
  #[inline]
  fn read<T>(&mut self) -> io::Result<T>
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

pub trait PrimitiveRead
{
  fn read_le<R>(reader: &mut R) -> io::Result<Self>
  where
    R: io::Read,
    Self: Sized;

  fn read_be<R>(reader: &mut R) -> io::Result<Self>
  where
    R: io::Read,
    Self: Sized;

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

pub trait Read
{
  fn read<R>(reader: &mut R) -> io::Result<Self>
  where
    R: io::Read,
    Self: Sized;
}

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
