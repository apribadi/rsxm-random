/*
use std::io::Write;
use quickrand::Rng;
use quickrand::RngState;

#[inline]
fn widening_mul(x: u64, y: u64) -> (u64, u64) {
  let z = (x as u128) * (y as u128);
  return (z as u64, (z >> 64) as u64);
}

#[inline] 
fn mum(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u ^ v);
}

fn f1(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u + v);
}

fn f2(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u ^ v);
}

fn f3(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u + v) + x;
}

fn f4(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u + v) ^ x;
}

fn f5(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u ^ v) + x;
}

fn f6(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u ^ v) ^ x;
}

fn f7(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u ^ v) + (x + y);
}

fn f8(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u ^ v) + (x ^ y);
}

fn f9(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u ^ v) ^ (x ^ y);
}

fn f0(x: u64, y: u64) -> u64 {
  let (u, v) = widening_mul(x, y);
  return (u ^ v) - (x ^ y);
}


fn put(x: u64) {
  print!("{:#066b} {:#018x}\n", x, x);
}

fn test<F>(f: F) where F: Fn(u64, u64) -> u64 {
  put(f(0xcafebabedeadbeef, (-1i64 as u64) >> 1));
  put(f((-1i64 as u64) >> 1, 0xcafebabedeadbeef));
  put(f(0xcafebabedeadbeef,  1 << 63));
  put(f(1 << 63 , 0xcafebabedeadbeef));
  put(f(0xcafebabedeadbeef, -3i64 as u64));
  put(f(0xcafebabedeadbeef, -2i64 as u64));
  put(f(0xcafebabedeadbeef, -1i64 as u64));
  put(f(0xcafebabedeadbeef, 0i64 as u64));
  put(f(0xcafebabedeadbeef, 1i64 as u64));
  put(f(0xcafebabedeadbeef, 2i64 as u64));
  put(f(0xcafebabedeadbeef, 3i64 as u64));
  put(f(-3i64 as u64, 0xcafebabedeadbeef));
  put(f(-2i64 as u64, 0xcafebabedeadbeef));
  put(f(-1i64 as u64, 0xcafebabedeadbeef));
  put(f(1i64 as u64, 0xcafebabedeadbeef));
  put(f(2i64 as u64, 0xcafebabedeadbeef));
  put(f(3i64 as u64, 0xcafebabedeadbeef));
  put(f(0xcafebabedeadbeef, 0xcafebabedeadbeef));
  println!("");
}

fn main() {
  // let mut gen = Rng::new(RngState::from_seed(0xcafebabe_deadbeef_12345678_90123456));
  let mut gen = Rng::new(RngState::from_seed(0));

  if true {
    test(f1);
    test(f2);
    test(f3);
    test(f4);
    test(f5);
    test(f6);
    test(f7);
    test(f8);
    test(f9);
    test(f0);

    for _ in 0 .. 50 {
      print!("{:#066b}\n", gen.u64());
    }
  } else {
    let mut buf = vec![0u8; 1024 * 8].into_boxed_slice();

    loop {
      for chunk in buf.chunks_mut(8) {
        let x = u64::to_le_bytes(gen.u64());
        chunk.copy_from_slice(&x);
      }

      match std::io::stdout().write_all(&buf) {
        Ok(()) => (),
        Err(_) => panic!("write failed")
      }
    }
  }
  /*
  let len = 1 << 13;
  let mut buf1 = vec![0u64; len].into_boxed_slice();
  // let mut gen = RandGen::new(RandState::from_seed(0x99999999_deadbeef_99999999_cafebabe));
  let mut gen = RandGen::new(RandState::from_seed(0));
  loop {
    fill(&mut gen, buf1.as_mut());
    LittleEndian::write_u64_into(buf1.as_mut(), buf2.as_mut());
  }
  */
}

fn fill(rng: &mut Rng, buf: &mut [u64]) {
  let mut state = rng.get_state();
  for slot in buf.iter_mut() {
    let (s, x) = state.u64();
    *slot = x;
    state = s;
  }
  rng.set_state(state);
}
*/

fn main() {
}
