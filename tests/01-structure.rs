use structurs::Read;

#[derive(Read, Default, Debug)]
pub struct TestData
{
  pub field_1: u32,
  pub field_2: i128,
  field_3: u8,
  pad_to_32: [u8; 11],
}

fn main()
{
  let v = vec![];
  let mut c = std::io::Cursor::new(v);
  let t = TestData::read(&mut c);
  eprintln!("{:#?}", t);
}
