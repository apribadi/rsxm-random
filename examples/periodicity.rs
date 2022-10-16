use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct M64([u64; 64]);

impl core::ops::Add for M64 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    M64::from_rows(core::array::from_fn(|i| self.row(i) ^ other.row(i)))
  }
}

impl core::ops::Mul for M64 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    let mut z = [0u64; 64];
    let x = self.rows();
    let y = other.cols();

    for i in 0 .. 64 {
      let mut r = 0u64;
      let u = x[i];
      for j in 0 .. 64 {
        let v = y[j];
        r |= (((u & v).count_ones() as u64) & 1) << j;
      }
      z[i] = r;
    }

    M64::from_rows(z)
  }
}

impl M64 {
  pub const ZERO: Self = M64([0u64; 64]);

  #[inline]
  pub fn from_rows(rows: [u64; 64]) -> Self {
    M64(rows)
  }

  #[inline]
  pub fn from_cols(rows: [u64; 64]) -> Self {
    Self::from_rows(rows).transpose()
  }

  #[inline]
  pub fn row(self, i: usize) -> u64 {
    assert!(i < 64);
    let M64(rows) = self;
    rows[i]
  }

  #[inline]
  pub fn rows(self) -> [u64; 64] {
    let M64(rows) = self;
    rows
  }

  #[inline]
  pub fn col(self, i: usize) -> u64 {
    assert!(i < 64);
    let mut c = 0u64;
    for j in 0 .. 64 {
      c |= ((self.row(j) >> i) & 1) << j;
    }
    c
  }

  #[inline]
  pub fn cols(self) -> [u64; 64] {
    core::array::from_fn(|i| self.col(i))
  }

  #[inline]
  pub fn transpose(self) -> Self {
    M64(self.cols())
  }

  #[inline]
  pub fn apply(self, x: u64) -> u64 {
    let mut y: u64 = 0;
    for i in 0 .. 64 {
      y |= (((self.row(i) ^ x).count_ones() as u64) & 1) << i;
    }
    y
  }

  #[inline]
  pub fn is_zero(self) -> bool {
    self.rows().iter().all(|row| *row == 0u64)
  }

  #[inline]
  pub fn is_id(self) -> bool {
    self.rows().iter().enumerate().all(|(i, row)| *row == 1u64 << i)
  }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct M128 {
  q00: M64,
  q01: M64,
  q10: M64,
  q11: M64,
}

impl core::ops::Add for M128 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    M128 {
      q00: self.q00 + other.q00,
      q01: self.q01 + other.q01,
      q10: self.q10 + other.q10,
      q11: self.q11 + other.q11,
    }
  }
}

impl core::ops::Mul for M128 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    M128 {
      q00: self.q00 * other.q00 + self.q01 * other.q10,
      q01: self.q00 * other.q01 + self.q01 * other.q11,
      q10: self.q10 * other.q00 + self.q11 * other.q10,
      q11: self.q10 * other.q01 + self.q11 * other.q11,
    }
  }
}

fn cat(x: u64, y: u64) -> u128 {
  (x as u128) | ((y as u128) << 64)
}


impl M128 {
  pub fn from_rows(rows: [u128; 128]) -> Self {
    M128 {
      q00: M64::from_rows(core::array::from_fn(|i| rows[i] as u64)),
      q01: M64::from_rows(core::array::from_fn(|i| (rows[i] >> 64) as u64)),
      q10: M64::from_rows(core::array::from_fn(|i| rows[i + 64] as u64)),
      q11: M64::from_rows(core::array::from_fn(|i| (rows[i + 64] >> 64) as u64)),
    }
  }

  pub fn from_cols(cols: [u128; 128]) -> Self {
    Self::from_rows(cols).transpose()
  }

  pub fn row(self, i: usize) -> u128 {
    assert!(i < 128);
    if i < 64 {
      cat(self.q00.row(i), self.q01.row(i))
    } else {
      cat(self.q10.row(i - 64), self.q11.row(i - 64))
    }
  }

  pub fn transpose(self) -> Self {
    M128 {
      q00: self.q00.transpose(),
      q01: self.q10.transpose(),
      q10: self.q01.transpose(),
      q11: self.q11.transpose(),
    }
  }

  pub fn pow(self, n: u128) -> Self {
    let mut y = Self::from_rows(core::array::from_fn(|i| 1u128 << i));
    let mut a = self;
    let mut n = n;

    while n != 0 {
      if n & 1 != 0 {
        y = y * a;
      }
      a = a * a;
      n = n >> 1;
    }

    y
  }

  pub fn is_id(self) -> bool {
    self.q00.is_id()
      && self.q01.is_zero()
      && self.q10.is_zero()
      && self.q11.is_id()
  }

  fn from_action_on_standard_basis<F>(f: F) -> M128 where F: Fn(u128) -> u128 {
    M128::from_cols(core::array::from_fn(|i| f(1 << i)))
  }

  fn from_action_on_standard_basis_u64<F>(f: F) -> M128 where F: Fn(u64, u64) -> (u64, u64) {
    M128::from_action_on_standard_basis(|x| {
      let x0 = x as u64;
      let x1 = (x >> 64) as u64;
      let (y0, y1) = f(x0, x1);
      let y = (y0 as u128) | ((y1 as u128) << 64);
      y
    })
  }
}

impl fmt::Display for M128 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for i in 0 .. 128 {
      for j in 0 .. 128 {
        let x = (self.row(i) >> j) & 1;
        write!(f, "{}", x)?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

const MAX_PERIOD: u128 = u128::MAX;

const FACTORS: [u128; 9] = [
  3,                  // F0
  5,                  // F1
  17,                 // F2
  257,                // F3
  65_537,             // F4
  641,                // F5 (1st)
  6_700_417,          // F5 (2nd)
  274_177,            // F6 (1st)
  67_280_421_310_721, // F6 (2nd)
];

fn check(a: M128) -> bool {
  let mut x = a;
  for _ in 0 .. 128 { x = x * x }
  x == a && {
    print!("checking ...\n");
    let mut is_ok = true;
    for d in FACTORS.iter() {
      let k = MAX_PERIOD / d;
      assert!(k * d == MAX_PERIOD);
      if a.pow(k).is_id() {
        print!("failed - {}\n", d);
        is_ok = false;
      }
    }
    is_ok
  }
}

fn main() {
  print!("hello!\n");
  {
    let a = 
      M128::from_action_on_standard_basis_u64(|x, y| {
        let u = x ^ y ^ x >> 5;
        let v = x ^ x << 23;
        let w = v ^ v >> 18;
        (u, w)
      });
    if check(a) {
      print!("xorshift128 has full period!\n");
    }
  }
  {
    let a = 
      M128::from_action_on_standard_basis_u64(|x, y|
        {
          let u = x.rotate_left(7) ^ y;
          let v = x ^ x << 19;
          (u, v)
        }
      );
    if check(a) {
      print!("full period!!\n");
      print!("{}\n\n\n", a);
      print!("{}\n\n\n", a * a);
      print!("{}\n\n\n", a * a * a);
      print!("{}\n\n\n", a * a * a * a);
    }
  }
}
