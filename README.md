# Structurs

A rust crate that allows you to read bytes into a structure from a source.

Writing a function that reads bytes into a structure can be a tedious job.
`#[derive(structurs::Read)]` macro automatically generates code that implements this trait for
your structure. It also includes attributes that can change the way some of the fields are
read.

By default all fields will be read using `structurs::Read::read` function, but you might have
fields that might need to be read in big-endian format. In that case you can mark those fields
with `#[be]` attribute.

# Attributes

The following is the list of attributes that can be used to mark the fields of structures.

- `#[le]`, This denotes that the field is in little-endian format.
- `#[be]`, This denotes that the field is in big-endian format.
- `#[ne]`, This denotes that the field is in CPU's native endian format. Most CPU's will use
  little-endian format.
- `#[pad]`, This denotes that the field is a padding and is not important. In this case the
  field will be initialized to its default value using `Default::default`. By default the
  length of the field type worth of bytes will be read from the reader. You can also pass a
  `bytes` value to this attribute. `#[pad(bytes = N)]` means that N bytes should be read from
  the reader in which case field type is not important and should be `structurs::Pad`.

# Example

```rust
#[derive(structurs::Read)]
struct Test
{
  // This field will be read using structurs::Read::read function.
  field_1: i64,
  // This field will be read using structurs::PrimitiveRead::read_le function.
  #[le]
  field_2: i16,
  // This field will not be read. But 8 bytes will be read from the reader and discarded.
  #[pad(bytes = 8)]
  pad_field: structurs::Pad,
}
```
