use expect_test::expect;
use quickrand::Rng;

#[test]
fn trivial_tests() {
  let mut g = Rng::new(42);
  expect!["0x5a2f8595cd4bbbe9"].assert_eq(&format!("{:#x}", g.u64()));
  expect!["0x3e812fb7a060460b"].assert_eq(&format!("{:#x}", g.i64()));
  expect!["0x1ece33b3"].assert_eq(&format!("{:#x}", g.u32()));
  expect!["0x63ec5784"].assert_eq(&format!("{:#x}", g.i32()));
  expect!["3"].assert_eq(&format!("{}", g.bounded_u32(10)));
  expect!["true"].assert_eq(&format!("{}", g.bool()));
}
