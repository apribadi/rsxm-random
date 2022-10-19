#![no_std]

#[derive(Clone)]
pub struct Rng {
  x: u64,
  y: u64,
}

#[inline]
const fn umulh(x: u64, y: u64) -> u64 {
  (((x as u128) * (y as u128)) >> 64) as u64
}

impl Rng {
  #[inline] 
  pub const fn next(Rng { x, y }: Rng) -> (Rng, u64) {
    let a = x.rotate_left(7) ^ y;
    let b = x ^ x << 19;
    let c = x.wrapping_mul(y) ^ umulh(x, y);
    let d = c.wrapping_add(a);
    (Rng { x: a, y: b }, d)
  }

  pub const fn new(seed: u128) -> Self {
    const M: u128 = 0x9e37_79b9_7f4a_7c15_f39c_c060_5ced_c835;
    let seed = seed.wrapping_mul(M);
    let seed = seed ^ seed >> 64;
    let seed = seed | 1;
    let seed = seed.wrapping_mul(M);
    let seed = seed ^ seed >> 64;
    let seed = seed.wrapping_mul(M);
    let seed = seed ^ seed >> 64;
    Rng { x: seed as u64, y: (seed >> 64) as u64 }
  }

  #[inline]
  pub fn u64(&mut self) -> u64 {
    let (s, x) = Self::next(self.clone());
    *self = s;
    x
  }

  #[inline]
  pub fn i64(&mut self) -> i64 {
    self.u64() as i64
  }

  #[inline]
  pub fn u32(&mut self) -> u32 {
    self.u64() as u32
  }

  #[inline]
  pub fn i32(&mut self) -> i32 {
    self.u64() as i32
  }

  #[inline]
  pub fn split(&mut self) -> Self {
    let x = self.u64() | 1;
    let y = self.u64();
    Rng { x, y }
  }

  #[inline]
  pub fn bool(&mut self) -> bool {
    self.i64() >= 0
  }

  #[inline]
  pub fn bounded_u32(&mut self, max: u32) -> u32 {
    umulh(self.u64(), (max as u64) + 1) as u32
  }

  pub fn fill(&mut self, dst: &mut [u8]) {
    let mut a = dst;

    if a.is_empty() { return; }

    loop {
      let x = self.u64().to_le_bytes();

      match a.len() {
        0 => { break; } // unreachable
        1 => { a.copy_from_slice(&x[.. 1]); break; }
        2 => { a.copy_from_slice(&x[.. 2]); break; }
        3 => { a.copy_from_slice(&x[.. 3]); break; }
        4 => { a.copy_from_slice(&x[.. 4]); break; }
        5 => { a.copy_from_slice(&x[.. 5]); break; }
        6 => { a.copy_from_slice(&x[.. 6]); break; }
        7 => { a.copy_from_slice(&x[.. 7]); break; }
        8 => { a.copy_from_slice(&x[.. 8]); break; }
        _ => { }
      }

      let (b, c) = a.split_at_mut(8);
      a = c;
      b.copy_from_slice(&x);
    }
  }
}
