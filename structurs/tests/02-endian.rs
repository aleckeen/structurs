use structurs::Read;

#[derive(Read, Default, Debug, Eq, PartialEq)]
pub struct TestData
{
  #[be]
  pub field_1: u32,
  #[ne]
  pub field_2: i128,
  #[be]
  field_3: u8,
  pad_to_32: [u8; 11],
}

const DATA: TestData = TestData {
  field_1: 510745010,
  field_2: 101876807604715792753432598791754839769,
  field_3: 100,
  pad_to_32: [15, 49, 94, 129, 93, 34, 122, 135, 84, 19, 162],
};

const DATA_BYTES: [u8; 32] = [
  30, 113, 89, 178, 217, 118, 243, 7, 67, 25, 132, 7, 240, 193, 119, 176, 106, 194, 164, 76, 100, 15, 49, 94, 129, 93,
  34, 122, 135, 84, 19, 162,
];

fn main()
{
  // random numbers to test. Not very useful but at least doesn't throw an error.
  let mut c = std::io::Cursor::new(DATA_BYTES);
  let t = TestData::read(&mut c).unwrap();
  assert_eq!(DATA, t);
}
