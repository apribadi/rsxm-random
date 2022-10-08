#![feature(bench_black_box)]

use std::time::Instant;
use std::hint::black_box;
use quickrand::Rng;

struct Xoroshiro128pp {
  a: u64,
  b: u64,
}

impl Xoroshiro128pp {
  fn new(seed: u128) -> Self {
    let seed = seed | 1;
    return Xoroshiro128pp { a: seed as u64, b: (seed >> 64) as u64 };
  }

  fn u64(&mut self) -> u64 {
    let a = self.a;
    let b = self.b;
    let x = a.wrapping_add(b).rotate_left(17).wrapping_add(a);
    let b = a ^ b;
    self.a = a.rotate_left(49) ^ b ^ b << 21;
    self.b = b.rotate_left(28);
    return x;
  }
}

fn run_bench<F>(name: &str, f: F) where F: Fn(u128) -> u64 {
  let start = Instant::now();
  let _: u64 = black_box(f(42));
  let stop = Instant::now();
  let elapsed = stop.saturating_duration_since(start).as_nanos();
  let per_word = (elapsed as f64) / 1_000_000_000.;
  print!("{}: {:?} / word\n", name, per_word);
}

#[inline(never)]
fn bench_xsmum128(seed: u128) -> u64 {
  let mut g = Rng::new(seed);
  let mut s = 0u64;
  for _ in 0 .. 1_000_000_000 {
    s = s.wrapping_add(g.u64());
  }
  return s;
}

#[inline(never)]
fn bench_xsmum128x2(seed: u128) -> u64 {
  let mut g0 = Rng::new(seed);
  let mut g1 = Rng::new(seed + 1);
  let mut s = 0u64;
  for _ in 0 .. 500_000_000 {
    s = s.wrapping_add(g0.u64());
    s = s.wrapping_add(g1.u64());
  }
  return s;
}

#[inline(never)]
fn bench_xsmum128x4(seed: u128) -> u64 {
  let mut g0 = Rng::new(seed);
  let mut g1 = Rng::new(seed + 1);
  let mut g2 = Rng::new(seed + 2);
  let mut g3 = Rng::new(seed + 3);
  let mut s = 0u64;
  for _ in 0 .. 250_000_000 {
    s = s.wrapping_add(g0.u64());
    s = s.wrapping_add(g1.u64());
    s = s.wrapping_add(g2.u64());
    s = s.wrapping_add(g3.u64());
  }
  return s;
}

#[inline(never)]
fn bench_xoroshiro128pp(seed: u128) -> u64 {
  let mut g = Xoroshiro128pp::new(seed);
  let mut s = 0u64;
  for _ in 0 .. 1_000_000_000 {
    s = s.wrapping_add(g.u64());
  }
  return s;
}

fn main() {
  run_bench("xsmum128", bench_xsmum128);
  run_bench("xsmum128x2", bench_xsmum128x2);
  run_bench("xsmum128x4", bench_xsmum128x4);
  run_bench("xoroshiro128++", bench_xoroshiro128pp);
}
