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
fn mul_lo_hi(x: u64, y: u64) -> (u64, u64) {
  let z = (x as u128) * (y as u128);
  return (z as u64, (z >> 64) as u64);
}

#[inline]
fn mul_hi(x: u64, y: u64) -> u64 {
  return (((x as u128) * (y as u128)) >> 64) as u64;
}

#[inline] 
fn mum(x: u64, y: u64) -> u64 {
  let (u, v) = mul_lo_hi(x, y);
  return u + v;
}

#[inline]
fn bits_to_range_u64(x: u64, lo: u64, hi: u64) -> u64 {
  // TODO: de-bias
  let u = hi.wrapping_sub(lo);
  let v = lo.wrapping_add(mul_hi(x, u.wrapping_add(1)));
  return if u == u64::MAX { x } else { v };
}

#[inline]
fn bits_to_range_u32(x: u64, lo: u32, hi: u32) -> u32 {
  let u = hi.wrapping_sub(lo);
  let v = lo.wrapping_add(mul_hi(x, (u as u64) + 1) as u32);
  return v;
}

#[inline]
fn bits_to_open01_f64(x: u64) -> f64 {
  return f64::from_bits(((1022 - x.trailing_zeros()) as u64) << 52 | x >> 12);
}

#[inline]
fn bits_to_open01_f32(x: u64) -> f32 {
  return f32::from_bits((((126 - x.trailing_zeros()) as u64) << 23 | x >> 41) as u32);
}

#[inline]
fn bits_to_bernoulli(x: u64, p: f64) -> bool {
  return bits_to_open01_f64(x) < p;
}

impl RandState {
  #[inline]
  fn next(self) -> (Self, u64) {
    let RandState { a, b } = self;
    let x = mum(a, b);
    let a = a | a << 7;
    let a = a | a >> 9;
    let b = b.wrapping_mul(5).wrapping_add(0x653);
    let s = RandState { a, b };
    return (s, x);
  }

  #[inline]
  fn step_one<F, A>(self, f: F) -> (Self, A) where F: FnOnce(u64) -> A {
    let (s, x) = self.next();
    return (s, f(x));
  }

  #[inline]
  fn step_two<F, A>(self, f: F) -> (Self, A) where F: FnOnce(u64, u64) -> A {
    let (s, x) = self.next();
    let (s, y) = s.next();
    return (s, f(x, y));
  }

  pub fn from_seed(seed: u128) -> Self {
    // let seed = hash(seed);
    let x = seed | 1;
    let a = x as u64;
    let b = (x >> 64) as u64;
    return RandState { a, b };
  }

  pub fn split(self) -> (Self, Self) {
    return self.step_two(|x, y| RandState { a: x | 1, b: y });
  }

  pub fn bool(self) -> (Self, bool) {
    return self.step_one(|x| x & 1 != 0);
  }

  pub fn bernoulli(self, p: f64) -> (Self, bool) {
    return self.step_one(|x| bits_to_bernoulli(x, p));
  }

  pub fn u64(self) -> (Self, u64) {
    return self.next();
  }

  pub fn i64(self) -> (Self, i64) {
    return self.step_one(|x| x as i64);
  }

  pub fn u32(self) -> (Self, u32) {
    return self.step_one(|x| x as u32);
  }

  pub fn i32(self) -> (Self, i32) {
    return self.step_one(|x| x as u32 as i32);
  }

  pub fn range_u64(self, lo: u64, hi: u64) -> (Self, u64) {
    return self.step_one(|x| bits_to_range_u64(x, lo, hi));
  }

  pub fn range_i64(self, lo: i64, hi: i64) -> (Self, i64) {
    return self.step_one(|x| bits_to_range_u64(x, lo as u64, hi as u64) as i64);
  }

  pub fn range_u32(self, lo: u32, hi: u32) -> (Self, u32) {
    return self.step_one(|x| bits_to_range_u32(x, lo, hi));
  }

  pub fn range_i32(self, lo: i32, hi: i32) -> (Self, i32) {
    return self.step_one(|x| bits_to_range_u32(x, lo as u32, hi as u32) as i32);
  }

  pub fn open01_f64(self) -> (Self, f64) {
    return self.step_one(bits_to_open01_f64);
  }

  pub fn open01_f32(self) -> (Self, f32) {
    return self.step_one(bits_to_open01_f32);
  }

  pub fn fill_u64(self, a: &mut [u64]) -> Self {
    let mut state = self;
    for slot in a.iter_mut() {
      let (s, x) = state.next();
      state = s;
      *slot = x;
    };
    return state;
  }
}

impl RandGen {
  pub fn new(s: RandState) -> Self {
    return RandGen { state: s };
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
  fn step<F, A>(&mut self, f: F) -> A where F: FnOnce(RandState) -> (RandState, A) {
    let (s, x) = f(self.get_state());
    self.set_state(s);
    return x;
  }

  pub fn split(&mut self) -> Self {
    return RandGen { state: self.step(RandState::split) };
  }

  pub fn bool(&mut self) -> bool {
    return self.step(RandState::bool);
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

  pub fn range_u32(&mut self, lo: u32, hi: u32) -> u32 {
    return self.step(|s| RandState::range_u32(s, lo, hi));
  }

  pub fn range_i32(&mut self, lo: i32, hi: i32) -> i32 {
    return self.step(|s| RandState::range_i32(s, lo, hi));
  }

  pub fn open01_f64(&mut self) -> f64 {
    return self.step(RandState::open01_f64);
  }

  pub fn open01_f32(&mut self) -> f32 {
    return self.step(RandState::open01_f32);
  }

  pub fn fill_u64(&mut self, a: &mut [u64]) {
    self.set_state(self.get_state().fill_u64(a));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut s = RandGen::new(RandState::from_seed(100));
    let a = s.u64();
    let b = s.u64();
    let result = a.wrapping_add(b);
    assert_eq!(result, 32071221);
  }
}
