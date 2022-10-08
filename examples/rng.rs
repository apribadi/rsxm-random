use std::io::Write;
use quickrand::Rng;

fn main() {
  let mut rng = Rng::new(42);
  let mut buf = vec![0u8; 1024 * 8].into_boxed_slice();
  let mut out = std::io::stdout().lock();

  loop {
    for chunk in buf.chunks_mut(8) {
      chunk.copy_from_slice(&u64::to_le_bytes(rng.u64()));
    }

    out.write_all(&buf).expect("write_all failed!");
  }
}
