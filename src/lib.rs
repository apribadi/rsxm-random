#![no_std]

#[derive(Clone)]
pub struct RandState {
  a: u64,
  b: u64,
}

#[derive(Clone)]
pub struct RandGen {
  state: RandState,
}

#[inline] 
fn mum(x: u64, y: u64) -> u64 {
  let z = (x as u128) * (y as u128);
  return (z as u64).wrapping_add((z >> 64) as u64);
}

#[inline]
fn next(RandState { a, b }: RandState) -> (RandState, u64) {
  let x = mum(a, b);
  let a = a | a << 7;
  let a = a | a >> 9;
  let b = b.wrapping_mul(5).wrapping_add(0x999);
  let s = RandState { a, b };
  return (s, x);
}

impl RandState {
  pub fn new(seed: u128) -> Self {
    // let seed = hash(seed);
    let x = seed | 1;
    let a = x as u64;
    let b = (x >> 64) as u64;
    return RandState { a, b };
  }

  pub fn u64(self) -> (Self, u64) {
    return next(self);
  }

  pub fn i64(self) -> (Self, i64) {
    let (s, x) = next(self);
    return (s, x as i64);
  }

  pub fn u32(self) -> (Self, u32) {
    let (s, x) = next(self);
    return (s, x as u32);
  }

  pub fn i32(self) -> (Self, i32) {
    let (s, x) = next(self);
    return (s, x as u32 as i32);
  }

  pub fn open01_f64(self) -> (Self, f64) {
    let (s, x) = next(self);
    let k = x.trailing_zeros() as u64;
    let x = (1022 - k) << 52 | x >> 12;
    let x = f64::from_bits(x);
    return (s, x);
  }

  pub fn open01_f32(self) -> (Self, f32) {
    let (s, x) = next(self);
    let k = x.trailing_zeros() as u64;
    let x = (126 - k) << 23 | x >> 41;
    let x = f32::from_bits(x as u32);
    return (s, x);
  }

  pub fn split(self) -> (Self, Self) {
    let (s, x) = next(self);
    let (s, y) = next(s);
    let a = x | 1;
    let b = y;
    return (s, RandState { a, b });
  }
}

impl RandGen {
  pub fn new(seed: u128) -> Self {
    return RandGen { state: RandState::new(seed) };
  }

  #[inline]
  pub fn get_state(&self) -> RandState {
    return self.state.clone();
  }

  #[inline]
  pub fn set_state(&mut self, s: RandState) {
    self.state = s;
  }

  #[inline]
  fn step<F, A>(&mut self, f: F) -> A where F: FnOnce(RandState)-> (RandState, A) {
    let (s, x) = f(self.get_state());
    self.set_state(s);
    return x;
  }

  pub fn u64(&mut self) -> u64 {
    return self.step(RandState::u64);
  }

  pub fn i64(&mut self) -> i64 {
    return self.step(RandState::i64);
  }

  pub fn u32(&mut self) -> u32 {
    return self.step(RandState::u32);
  }

  pub fn i32(&mut self) -> i32 {
    return self.step(RandState::i32);
  }

  pub fn open01_f64(&mut self) -> f64 {
    return self.step(RandState::open01_f64);
  }

  pub fn open01_f32(&mut self) -> f32 {
    return self.step(RandState::open01_f32);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let s = RandState::new(100);
    let _ = next(s.clone());
    let _ = next(s);
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
