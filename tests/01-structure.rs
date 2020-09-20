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
  // random numbers to test. Not very useful but at least doesn't throw an error.
  let v = vec![
    30, 113, 89, 178, 217, 118, 243, 7, 67, 25, 132, 7, 240, 193, 119, 176, 106, 194, 164, 76, 100, 15, 49, 94, 129,
    93, 34, 122, 135, 84, 19, 162,
  ];
  let mut c = std::io::Cursor::new(v);
  let t = TestData::read(&mut c);
  eprintln!("{:#?}", t);
}
