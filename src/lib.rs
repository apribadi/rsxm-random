#![no_std]

#[derive(Clone)]
pub struct Rng {
  a: u64,
  b: u64,
}

#[inline]
fn umulh(x: u64, y: u64) -> u64 {
  return (((x as u128) * (y as u128)) >> 64) as u64;
}

#[inline] 
fn next(Rng { a, b }: Rng) -> (Rng, u64) {
  let x = a.wrapping_mul(b) ^ umulh(a, b);
  let c = b ^ b << 9;
  let d = a ^ a >> 7;
  return (Rng { a: c, b: d }, x);
}

impl Rng {
  #[inline]
  pub fn new(seed: u128) -> Self {
    const M: u128 = 0x9e37_79b9_7f4a_7c15_f39c_c060_5ced_c835;
    let seed = if seed == 0 { 0x0bad_5eed } else { seed };
    let seed = seed.wrapping_mul(M);
    let seed = seed ^ seed >> 64;
    let seed = seed.wrapping_mul(M);
    let seed = seed ^ seed >> 64;
    let seed = seed.wrapping_mul(M);
    let seed = seed ^ seed >> 64;
    let a = seed as u64;
    let b = (seed >> 64) as u64;
    let a = if a == 0 { 0x0bad_5eed } else { a };
    let b = if b == 0 { 0x0bad_5eed } else { b };
    return Rng { a, b };
  }

  #[inline]
  pub fn set(&mut self, state: Self) {
    core::mem::drop(core::mem::replace(self, state));
  }

  #[inline]
  pub fn u64(&mut self) -> u64 {
    let (s, x) = next(self.clone());
    self.set(s);
    return x;
  }

  #[inline]
  pub fn split(&mut self) -> Self {
    let a = self.u64() | 1;
    let b = self.u64() | 1;
    return Rng { a, b };
  }

  #[inline]
  pub fn bool(&mut self) -> bool {
    return self.u64() >> 63 != 0;
  }

  #[inline]
  pub fn i64(&mut self) -> i64 {
    return self.u64() as i64;
  }

  #[inline]
  pub fn u32(&mut self) -> u32 {
    return (self.u64() >> 32) as u32;
  }

  #[inline]
  pub fn i32(&mut self) -> i32 {
    return (self.u64() >> 32) as i32;
  }

  #[inline]
  pub fn range_u64(&mut self, lo: u64, hi: u64) -> u64 {
    // TODO: de-bias
    let x = self.u64();
    let u = hi.wrapping_sub(lo).wrapping_add(1);
    let v = umulh(x, u);
    let w = if u == 0 { x } else { v };
    return lo.wrapping_add(w);
  }

  #[inline]
  pub fn range_i64(&mut self, lo: i64, hi: i64) -> i64 {
    return self.range_u64(lo as u64, hi as u64) as i64;
  }

  #[inline]
  pub fn range_u32(&mut self, lo: u32, hi: u32) -> u32 {
    let x = self.u64();
    let y = hi.wrapping_sub(lo);
    let z = lo.wrapping_add(umulh(x, (y as u64) + 1) as u32);
    return z;
  }

  #[inline]
  pub fn range_i32(&mut self, lo: i32, hi: i32) -> i32 {
    return self.range_u32(lo as u32, hi as u32) as i32;
  }

  #[inline]
  pub fn open01_f64(&mut self) -> f64 {
    let x = self.u64();
    let y = f64::from_bits(((1022 - x.trailing_zeros()) as u64) << 52 | x >> 12);
    return y;
  }

  #[inline]
  pub fn open01_f32(&mut self) -> f32 {
    let x = self.u64();
    let y = f32::from_bits((((126 - x.trailing_zeros()) as u64) << 23 | x >> 41) as u32);
    return y;
  }

  #[inline]
  pub fn bernoulli(&mut self, p: f64) -> bool {
    return self.open01_f64() < p;
  }

}
