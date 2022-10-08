#[inline]
fn mul_hi(x: u64, y: u64) -> u64 {
  return (((x as u128) * (y as u128)) >> 64) as u64;
}

fn f0(x: u64, y: u64) -> u64 {
  return x.wrapping_mul(y) ^ mul_hi(x, y);
}

fn f1(x: u64, y: u64) -> u64 {
  return x.wrapping_mul(y) + mul_hi(x, y);
}

fn f2(x: u64, y: u64) -> u64 {
  return x.wrapping_mul(y) - mul_hi(x, y);
}

fn check<F>(f: F) where F: Fn(u64, u64) -> u64 {
  print!("{:#018x}\n", f(-2i64 as u64, 0xdead_beef));
  print!("{:#018x}\n", f(-1i64 as u64, 0xdead_beef));
  print!("{:#018x}\n", f(1, 0xdead_beef));
  print!("{:#018x}\n", f(2, 0xdead_beef));
  print!("\n");

}

fn main() {
  check(f0);
  check(f1);
  check(f2);
}
