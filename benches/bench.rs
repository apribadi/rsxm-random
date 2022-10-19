#![feature(bench_black_box)]

use std::time::Instant;
use std::hint::black_box;
use quickrand::Rng;

const NUM_ITERATIONS: i64 = 100_000_000;

pub trait RngForBench {
  fn new(seed: u128) -> Self;

  fn u64(&mut self) -> u64;

  #[inline(never)]
  fn u64_noinline(&mut self) -> u64 {
    self.u64()
  }
}

impl RngForBench for Rng {
  fn new(seed: u128) -> Self {
    Self::new(seed)
  }

  #[inline]
  fn u64(&mut self) -> u64 {
    self.u64()
  }
}

struct Xoroshiro128pp {
  x: u64,
  y: u64,
}

impl RngForBench for Xoroshiro128pp {
  fn new(seed: u128) -> Self {
    Xoroshiro128pp { x: (seed as u64) | 1, y: ((seed >> 64) as u64) | 1 }
  }

  #[inline]
  fn u64(&mut self) -> u64 {
    let x = self.x;
    let y = self.y;
    let z = x.wrapping_add(y).rotate_left(17).wrapping_add(x);
    let y = x ^ y;
    let x = x.rotate_left(49) ^ y ^ y << 21;
    let y = y.rotate_left(28);
    self.x = x;
    self.y = y;
    z
  }
}

struct Pcg64dxsm {
  x: u128,
}

impl RngForBench for Pcg64dxsm {
  fn new(seed: u128) -> Self {
    Pcg64dxsm { x: seed }
  }

  #[inline]
  fn u64(&mut self) -> u64 {
    let x = self.x;
    let a = x as u64;
    let b = (x >> 64) as u64;
    let a = a | 1;
    let b = b ^ b >> 32;
    let b = b * 0xda942042e4dd58b5;
    let b = b ^ b >> 48;
    let b = b * a;
    let x = 0xda942042e4dd58b5 * x + 1;
    self.x = x;
    b
  }
}

struct RomuDuo {
  x: u64,
  y: u64,
}

impl RngForBench for RomuDuo {
  fn new(seed: u128) -> Self {
    RomuDuo { x: (seed as u64) | 1, y: ((seed >> 64) as u64) | 1 }
  }

  #[inline]
  fn u64(&mut self) -> u64 {
    let x = self.x;
    let y = self.y;
    let z = x;
    let x = 15241094284759029579 * y;
    let y = y.rotate_left(36).wrapping_add(y.rotate_left(15)).wrapping_sub(z);
    self.x = x;
    self.y = y;
    z
  }
}

fn run_bench<F>(name: &str, f: F) where F: Fn(u128) -> u64 {
  let start = Instant::now();
  let _: u64 = black_box(f(42));
  let stop = Instant::now();
  let elapsed = stop.saturating_duration_since(start).as_nanos();
  let per_word = (elapsed as f64) / (NUM_ITERATIONS as f64);
  print!("{}: {:?}ns / word\n", name, per_word);
}

fn run_bench_loop<T: RngForBench>(seed: u128) -> u64 {
  let mut g = T::new(seed);
  let mut s = 0u64;
  for _ in 0 .. NUM_ITERATIONS {
    s = s.wrapping_add(g.u64());
  }
  s
}

fn run_bench_loop_2x_rng<T: RngForBench>(seed: u128) -> u64 {
  let mut g0 = T::new(seed);
  let mut g1 = T::new(seed.wrapping_add(1));
  let mut s = 0u64;
  for _ in 0 .. NUM_ITERATIONS / 2 {
    s = s.wrapping_add(g0.u64());
    s = s.wrapping_add(g1.u64());
  }
  s
}

fn run_bench_loop_noinline<T: RngForBench>(seed: u128) -> u64 {
  let mut g = T::new(seed);
  let mut s = 0u64;
  for _ in 0 .. NUM_ITERATIONS {
    s = s.wrapping_add(g.u64_noinline());
  }
  s
}

// We define the below "bench_*" functions in such a way that we can easily
// read their generated assembly.

#[inline(never)]
fn bench_xmum128(seed: u128) -> u64 {
  run_bench_loop::<Rng>(seed)
}

#[inline(never)]
fn bench_xoroshiro128pp(seed: u128) -> u64 {
  run_bench_loop::<Xoroshiro128pp>(seed)
}

#[inline(never)]
fn bench_pcg64dxsm(seed: u128) -> u64 {
  run_bench_loop::<Pcg64dxsm>(seed)
}

#[inline(never)]
fn bench_romuduo(seed: u128) -> u64 {
  run_bench_loop::<RomuDuo>(seed)
}

#[inline(never)]
fn bench_xmum128_2x_rng(seed: u128) -> u64 {
  run_bench_loop_2x_rng::<Rng>(seed)
}

#[inline(never)]
fn bench_xoroshiro128pp_2x_rng(seed: u128) -> u64 {
  run_bench_loop_2x_rng::<Xoroshiro128pp>(seed)
}

#[inline(never)]
fn bench_pcg64dxsm_2x_rng(seed: u128) -> u64 {
  run_bench_loop_2x_rng::<Pcg64dxsm>(seed)
}

#[inline(never)]
fn bench_romuduo_2x_rng(seed: u128) -> u64 {
  run_bench_loop_2x_rng::<RomuDuo>(seed)
}

#[inline(never)]
fn bench_xmum128_noinline(seed: u128) -> u64 {
  run_bench_loop_noinline::<Rng>(seed)
}

#[inline(never)]
fn bench_xoroshiro128pp_noinline(seed: u128) -> u64 {
  run_bench_loop_noinline::<Xoroshiro128pp>(seed)
}

#[inline(never)]
fn bench_pcg64dxsm_noinline(seed: u128) -> u64 {
  run_bench_loop_noinline::<Pcg64dxsm>(seed)
}

#[inline(never)]
fn bench_romuduo_noinline(seed: u128) -> u64 {
  run_bench_loop_noinline::<RomuDuo>(seed)
}

fn main() {
  run_bench("xmum128", bench_xmum128);
  run_bench("xoroshiro128++", bench_xoroshiro128pp);
  run_bench("pcg64dxsm", bench_pcg64dxsm);
  run_bench("romuduo", bench_romuduo);
  run_bench("xmum128 (2x rng)", bench_xmum128_2x_rng);
  run_bench("xoroshiro128++ (2x rng)", bench_xoroshiro128pp_2x_rng);
  run_bench("pcg64dxsm (2x rng)", bench_pcg64dxsm_2x_rng);
  run_bench("romuduo (2x rng)", bench_romuduo_2x_rng);
  run_bench("xmum128 (noinline)", bench_xmum128_noinline);
  run_bench("xoroshiro128++ (noinline)", bench_xoroshiro128pp_noinline);
  run_bench("pcg64dxsm (noinline)", bench_pcg64dxsm_noinline);
  run_bench("romuduo (noinline)", bench_romuduo_noinline);
}
