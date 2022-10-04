use expect_test::expect;
use quickrand::Rng;

#[test]
fn trivial_tests() {
  let mut g = Rng::from_seed(42);
  expect!["false"].assert_eq(&format!("{}", g.bool()));
  expect!["false"].assert_eq(&format!("{}", g.bernoulli(0.25)));
  expect!["0x6b60963a12fc9044"].assert_eq(&format!("{:#x}", g.i64()));
  expect!["0x2afdba2b"].assert_eq(&format!("{:#x}", g.u32()));
  expect!["0xb5752741"].assert_eq(&format!("{:#x}", g.i32()));
  expect!["80"].assert_eq(&format!("{}", g.range_u64(50, 99)));
  expect!["75"].assert_eq(&format!("{}", g.range_i64(50, 99)));
  expect!["59"].assert_eq(&format!("{}", g.range_u32(50, 99)));
  expect!["92"].assert_eq(&format!("{}", g.range_i32(50, 99)));
  expect!["0.2575813687616931"].assert_eq(&format!("{}", g.open01_f64()));
  expect!["0.923265"].assert_eq(&format!("{}", g.open01_f32()));
}
