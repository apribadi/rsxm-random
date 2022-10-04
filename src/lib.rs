#[derive(Clone)]
pub struct RngState {
  a: u64,
  b: u64,
}

#[derive(Clone)]
pub struct Rng {
  state: RngState,
}

#[inline]
fn mul_hi(x: u64, y: u64) -> u64 {
  return (((x as u128) * (y as u128)) >> 64) as u64;
}

#[inline] 
fn next(RngState { a, b }: RngState) -> (RngState, u64) {
  let x = (a.wrapping_mul(b) ^ mul_hi(a, b)).wrapping_add(a ^ b);
  let c = a ^ b ^ a >> 5;
  let d = a ^ a << 23;
  let d = d ^ d >> 18;
  return (RngState { a: c, b: d }, x);
}

#[inline]
fn from_seed(seed: u128) -> RngState {
  const E: u128 = 0x0bad_5eed_0bad_5eed_0bad_5eed_0bad_5eed;
  const M: u128 = 0x9e37_79b9_7f4a_7c15_f39c_c060_5ced_c835;
  let seed = seed.wrapping_mul(M);
  let seed = seed ^ seed >> 64;
  let seed = seed.wrapping_mul(M);
  let seed = seed ^ seed >> 64;
  let seed = seed.wrapping_mul(M);
  let seed = seed ^ seed >> 64;
  let seed = if seed == 0 { E } else { seed };
  return RngState { a: seed as u64, b: (seed >> 64) as u64 };
}

#[inline]
fn split(s: RngState) -> (RngState, RngState) {
  let (s, a) = next(s);
  let (s, b) = next(s);
  return (s, RngState { a: a | 1, b });
}

#[inline]
fn bits_to_range_u64(x: u64, lo: u64, hi: u64) -> u64 {
  // TODO: de-bias
  let u = hi.wrapping_sub(lo).wrapping_add(1);
  let v = mul_hi(x, u);
  let w = if u == 0 { x } else { v };
  return lo.wrapping_add(w);
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

impl RngState {
  #[inline]
  fn step<F, A>(self, f: F) -> (Self, A) where F: FnOnce(u64) -> A {
    let (s, x) = next(self);
    return (s, f(x));
  }

  pub fn from_seed(seed: u128) -> Self {
    return from_seed(seed);
  }

  pub fn split(self) -> (Self, Self) {
    return split(self);
  }

  pub fn bool(self) -> (Self, bool) {
    return self.step(|x| x >> 63 != 0);
  }

  pub fn bernoulli(self, p: f64) -> (Self, bool) {
    return self.step(|x| bits_to_bernoulli(x, p));
  }

  pub fn u64(self) -> (Self, u64) {
    return next(self);
  }

  pub fn i64(self) -> (Self, i64) {
    return self.step(|x| x as i64);
  }

  pub fn u32(self) -> (Self, u32) {
    return self.step(|x| (x >> 32) as u32);
  }

  pub fn i32(self) -> (Self, i32) {
    return self.step(|x| (x >> 32) as i32);
  }

  pub fn range_u64(self, lo: u64, hi: u64) -> (Self, u64) {
    return self.step(|x| bits_to_range_u64(x, lo, hi));
  }

  pub fn range_i64(self, lo: i64, hi: i64) -> (Self, i64) {
    return self.step(|x| bits_to_range_u64(x, lo as u64, hi as u64) as i64);
  }

  pub fn range_u32(self, lo: u32, hi: u32) -> (Self, u32) {
    return self.step(|x| bits_to_range_u32(x, lo, hi));
  }

  pub fn range_i32(self, lo: i32, hi: i32) -> (Self, i32) {
    return self.step(|x| bits_to_range_u32(x, lo as u32, hi as u32) as i32);
  }

  pub fn open01_f64(self) -> (Self, f64) {
    return self.step(bits_to_open01_f64);
  }

  pub fn open01_f32(self) -> (Self, f32) {
    return self.step(bits_to_open01_f32);
  }
}

impl Rng {
  pub fn new(s: RngState) -> Self {
    return Rng { state: s };
  }

  #[inline]
  pub fn get_state(&self) -> RngState {
    return self.state.clone();
  }

  #[inline]
  pub fn set_state(&mut self, s: RngState) {
    self.state = s;
  }

  #[inline]
  fn step<F, A>(&mut self, f: F) -> A where F: FnOnce(RngState) -> (RngState, A) {
    let (s, x) = f(self.get_state());
    self.set_state(s);
    return x;
  }

  pub fn from_seed(seed: u128) -> Self {
    return Rng { state: RngState::from_seed(seed) };
  }

  pub fn split(&mut self) -> Self {
    return Rng { state: self.step(RngState::split) };
  }

  pub fn bool(&mut self) -> bool {
    return self.step(RngState::bool);
  }

  pub fn bernoulli(&mut self, p: f64) -> bool {
    return self.step(|s| RngState::bernoulli(s, p));
  }

  pub fn u64(&mut self) -> u64 {
    return self.step(RngState::u64);
  }

  pub fn i64(&mut self) -> i64 {
    return self.step(RngState::i64);
  }

  pub fn u32(&mut self) -> u32 {
    return self.step(RngState::u32);
  }

  pub fn i32(&mut self) -> i32 {
    return self.step(RngState::i32);
  }

  pub fn range_u64(&mut self, lo: u64, hi: u64) -> u64 {
    return self.step(|s| RngState::range_u64(s, lo, hi));
  }

  pub fn range_i64(&mut self, lo: i64, hi: i64) -> i64 {
    return self.step(|s| RngState::range_i64(s, lo, hi));
  }

  pub fn range_u32(&mut self, lo: u32, hi: u32) -> u32 {
    return self.step(|s| RngState::range_u32(s, lo, hi));
  }

  pub fn range_i32(&mut self, lo: i32, hi: i32) -> i32 {
    return self.step(|s| RngState::range_i32(s, lo, hi));
  }

  pub fn open01_f64(&mut self) -> f64 {
    return self.step(RngState::open01_f64);
  }

  pub fn open01_f32(&mut self) -> f32 {
    return self.step(RngState::open01_f32);
  }
}
