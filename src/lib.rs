#![no_std]

#[derive(Clone)]
pub struct RandomState {
  a: u64,
  b: u64,
}

#[inline] 
fn mum(x: u64, y: u64) -> u64 {
  let z = (x as u128) * (y as u128);
  return (z as u64) + ((z >> 64) as u64);
}

#[inline]
fn next(RandomState { a, b }: RandomState) -> (RandomState, u64) {
  let x = mum(a, b);
  let a = a | a << 7;
  let a = a | a >> 9;
  let b = b + 0x9999_9999_9999_9999;
  let s = RandomState { a, b };
  return (s, x);
}

impl RandomState {
  pub fn new(seed: u128) -> Self {
    // let seed = hash(seed);
    let x = seed | 1;
    return RandomState { a: x as u64, b: (x >> 64) as u64 };
  }

  pub fn uniform_u64(self) -> (Self, u64) {
    return next(self);
  }

  pub fn uniform_u32(self) -> (Self, u32) {
    let (s, x) = next(self);
    return (s, x as u32);
  }

  pub fn uniform_f64_open_01(self) -> (Self, f64) {
    let (s, x) = next(self);
    let x = 0x3FF0_0000_0000_0000 | x >> 12;
    let x = f64::from_bits(x) - f64::from_bits(0x3FEF_FFFF_FFFF_FFFF);
    return (s, x);
  }

  pub fn split(self) -> (Self, Self) {
    let (s, x) = next(self);
    let (s, y) = next(s);
    return (s, RandomState { a: x | 1, b: y });
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let s = RandomState::new(100);
    let _ = next(s.clone());
    let _ = next(s);
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
