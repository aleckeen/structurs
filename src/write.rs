use std::io;

pub trait Writer: io::Write
{
  #[inline]
  fn write_as<T>(&mut self, v: &T) -> io::Result<()>
  where
    T: Write,
    Self: Sized,
  {
    T::write(v, self)
  }

  #[inline]
  fn write_le<T>(&mut self, v: &T) -> io::Result<()>
  where
    T: PrimitiveWrite,
    Self: Sized,
  {
    T::write_le(v, self)
  }

  #[inline]
  fn write_be<T>(&mut self, v: &T) -> io::Result<()>
  where
    T: PrimitiveWrite,
    Self: Sized,
  {
    T::write_be(v, self)
  }
}

impl<T> Writer for T where T: io::Write {}

pub trait PrimitiveWrite
{
  fn write_le<W>(&self, writer: &mut W) -> io::Result<()>
  where
    W: io::Write,
    Self: Sized;

  fn write_be<W>(&self, writer: &mut W) -> io::Result<()>
  where
    W: io::Write,
    Self: Sized;

  #[cfg(target_endian = "little")]
  #[inline]
  fn write_ne<W>(&self, writer: &mut W) -> io::Result<()>
  where
    W: io::Write,
    Self: Sized,
  {
    Self::write_le(self, writer)
  }

  #[cfg(target_endian = "big")]
  #[inline]
  fn write_ne<W>(&self, writer: &mut W) -> io::Result<()>
  where
    W: io::Write,
    Self: Sized,
  {
    Self::write_be(self, writer)
  }
}

pub trait Write
{
  fn write<W>(&self, writer: &mut W) -> io::Result<()>
  where
    W: io::Write,
    Self: Sized;
}
