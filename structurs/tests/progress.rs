#[test]
fn tests()
{
  let t = trybuild::TestCases::new();
  t.pass("tests/01-structure.rs");
  t.pass("tests/02-endian.rs");
  t.pass("tests/03-padding.rs");
}