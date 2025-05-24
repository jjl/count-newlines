#![no_std]
#[cfg(target_arch="x86_64")]
use core::arch::x86_64::*;
#[cfg(all(target_arch="x86", target_feature="sse2"))]
use core::arch::x86::*;

/// Counts the number of newlines in a slice.
pub fn count_newlines(slice: &[u8]) -> usize {
  let mut count = 0;
  let mut slice = slice;
  #[cfg(target_feature="sse2")] {
    unsafe {
      let splat = _mm_set1_epi8(b'\n' as _);
      while slice.len() >= 64 {
        let ptr = slice.as_ptr().cast::<__m128i>();
        for i in 0..4 {
          let simd = _mm_loadu_si128(ptr.add(i));
          count += _mm_movemask_epi8(_mm_cmpeq_epi8(splat, simd)).count_ones() as usize;
        }
        slice = &slice[64..];
      }
      while slice.len() >= 16 {
        let bytes = _mm_loadu_si128(slice.as_ptr().cast());
        count += _mm_movemask_epi8(_mm_cmpeq_epi8(splat, bytes)).count_ones() as usize;
        slice = &slice[16..];
      }
    }
  }
  for c in slice {
    if *c == b'\n' {
      count += 1;
    }
  }
  count
}
