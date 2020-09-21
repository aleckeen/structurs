use structurs::{Pad, Reader};

#[derive(structurs::Read, Debug, Eq, PartialEq, Default)]
pub struct TestData2
{
  #[le]
  field_1: u32,
  #[le]
  field_2: i16,
}

#[derive(structurs::Read, Default, Debug, Eq, PartialEq)]
pub struct TestData
{
  #[be]
  pub field_1: u32,
  #[ne]
  pub field_2: i128,
  #[be]
  field_3: u8,
  #[pad(bytes = 11)]
  pad_to_32: Pad,
  test_data_2: TestData2,
  #[pad]
  another_pad: [u32; 12],
}

const DATA: TestData = TestData {
  field_1: 510745010,
  field_2: 101876807604715792753432598791754839769,
  field_3: 100,
  pad_to_32: Pad,
  test_data_2: TestData2 {
    field_1: 1222188209,
    field_2: -30174,
  },
  another_pad: [0; 12],
};

const DATA_BYTES: [u8; 104] = [
  30, 113, 89, 178, 217, 118, 243, 7, 67, 25, 132, 7, 240, 193, 119, 176, 106, 194, 164, 76, 100, 15, 49, 94, 129, 93,
  34, 122, 135, 84, 19, 162, 177, 28, 217, 72, 34, 138, 120, 126, 147, 167, 89, 14, 96, 133, 107, 66, 141, 244, 174,
  13, 60, 26, 52, 53, 123, 162, 196, 107, 33, 77, 222, 199, 147, 209, 31, 124, 70, 155, 1, 93, 120, 87, 128, 217, 184,
  128, 127, 232, 247, 25, 89, 43, 192, 212, 193, 177, 36, 197, 157, 140, 242, 208, 135, 155, 117, 114, 195, 215, 109,
  70, 234, 112,
];

fn main()
{
  // random numbers to test. Not very useful but at least doesn't throw an error.
  let mut c = std::io::Cursor::new(DATA_BYTES);
  let t = c.read::<TestData>().unwrap();
  assert_eq!(DATA, t);
}
