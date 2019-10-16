use std::arch::x86_64::*;
use std::cmp::*;

pub fn popcount_u64s(u64s: &[u64]) -> i32 {  // u64sの要素数は4の整数倍。
    unsafe {
        let table = _mm256_setr_epi8(0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4);
        let mask  = _mm256_set1_epi8(0x0f);
        let zero  = _mm256_setzero_si256();

        let mut r_64x4 = _mm256_setzero_si256();
        let mut r_8x32 = _mm256_setzero_si256();

        let mut c = 0;

        for i in (0..u64s.len()).step_by(4) {
            let l = _mm256_loadu_si256(u64s.as_ptr().add(i as usize) as *const __m256i);

            r_8x32 = _mm256_add_epi8(r_8x32, _mm256_add_epi8(_mm256_shuffle_epi8(table, _mm256_and_si256(                  l,     mask)),
                                                             _mm256_shuffle_epi8(table, _mm256_and_si256(_mm256_srli_epi64(l, 4), mask))));

            c += 1;

            if c == 31 {
                r_64x4 = _mm256_add_epi64(r_64x4, _mm256_sad_epu8(r_8x32, zero));
                r_8x32 = _mm256_setzero_si256();

                c = 0;
            }
        }

        if c != 0 {
            r_64x4 = _mm256_add_epi64(r_64x4, _mm256_sad_epu8(r_8x32, zero));
        }

        let r_64x2 = _mm_add_epi64(_mm256_extracti128_si256(r_64x4, 1), _mm256_castsi256_si128(r_64x4));
        let r_64x1 = _mm_add_epi64(_mm_srli_si128(r_64x2, 8), r_64x2);

        _mm_cvtsi128_si64(r_64x1) as i32
    }
}

#[derive(Clone)]
struct Bitmap {
    width: i32,
    height: i32,
    lines: Vec<u64>
}

impl Bitmap {
    fn new(mut lines: Vec<u64>, width: i32) -> Bitmap {
        Bitmap {
            width: width,
            height: lines.len() as i32,
            lines: {
                lines.reserve_exact(3);

                for _ in 0..3 {
                    lines.push(0);
                }

                lines
            }
        }
    }

    fn count(&self) -> i32 {
        popcount_u64s(&self.lines[0..((self.height + 3) & !0b0011) as usize])
    }
}

pub struct Stamp(Bitmap);

impl Stamp {
    pub fn new(lines: Vec<u64>, width: i32) -> Stamp {
        Stamp(Bitmap::new(lines, width))
    }

    pub fn width(&self) -> i32 {
        self.0.width
    }

    pub fn height(&self) -> i32 {
        self.0.height
    }

    pub fn lines(&self) -> &[u64] {
        &self.0.lines
    }

    pub fn count(&self) -> i32 {
        self.0.count()
    }
}

#[derive(Clone)]
pub struct FieldUnit(Bitmap);

impl FieldUnit {
    pub fn new(lines: Vec<u64>, width: i32) -> FieldUnit {
        FieldUnit(Bitmap::new(lines, width))
    }

    pub fn width(&self) -> i32 {
        self.0.width
    }

    pub fn height(&self) -> i32 {
        self.0.height
    }

    pub fn lines(&self) -> &[u64] {
        &self.0.lines
    }

    pub fn count(&self) -> i32 {
        self.0.count()
    }

    pub fn stamp(&mut self, stamp: &Stamp, x: i32, y: i32) {
        unsafe {
            let y_1 = max( y, 0);
            let y_2 = max(-y, 0);
            let height = stamp.0.height - y_2 - max(y_1 + stamp.0.height - self.0.height, 0);
            let m = _mm256_set1_epi64x(((0xffffffffffffffff as u64) >> (64 - self.0.width)) as i64);

            for i in (0..((height + 3) & !0b0011)).step_by(4) {
                let l_1 = _mm256_loadu_si256(self.0.lines.as_ptr().add((y_1 + i) as usize) as *const __m256i);
                let l_2 = {
                    let mut result = _mm256_loadu_si256(stamp.0.lines.as_ptr().add((y_2 + i) as usize) as *const __m256i);

                    result = if x > 0 {
                        _mm256_slli_epi64(result,  x)
                    } else {
                        _mm256_srli_epi64(result, -x)
                    };
                    result = _mm256_and_si256(result, m);

                    result
                };

                _mm256_storeu_si256(self.0.lines.as_ptr().add((y_1 + i) as usize) as *mut __m256i, _mm256_xor_si256(l_1, l_2));
            }
        }

        self.0.lines[ self.0.height      as usize] = 0;
        self.0.lines[(self.0.height + 1) as usize] = 0;
        self.0.lines[(self.0.height + 2) as usize] = 0;
    }

    fn stamp_on_top_with_mask(&mut self, stamp: &Stamp, x: i32) {
        unsafe {
            let height = min(self.0.height, stamp.0.height);
            let m = _mm256_set1_epi64x(((0xffffffffffffffff as u64) >> (64 - self.0.width)) as i64);

            for i in (0..((height + 3) & !0b0011)).step_by(4) {
                let l_1 = _mm256_loadu_si256(self.0.lines.as_ptr().add(i as usize) as *const __m256i);
                let l_2 = {
                    let mut result = _mm256_loadu_si256(stamp.0.lines.as_ptr().add(i as usize) as *const __m256i);

                    result = if x > 0 {
                        _mm256_slli_epi64(result,  x)
                    } else {
                        _mm256_srli_epi64(result, -x)
                    };
                    result = _mm256_and_si256(result, m);

                    result
                };

                _mm256_storeu_si256(self.0.lines.as_ptr().add(i as usize) as *mut __m256i, _mm256_xor_si256(l_1, l_2));
            }
        }

        self.0.lines[ self.0.height      as usize] = 0;
        self.0.lines[(self.0.height + 1) as usize] = 0;
        self.0.lines[(self.0.height + 2) as usize] = 0;
    }

    fn stamp_on_top_without_mask(&mut self, stamp: &Stamp, x: i32) {
        unsafe {
            let height = min(self.0.height, stamp.0.height);

            for i in (0..((height + 3) & !0b0011)).step_by(4) {
                let l_1 = _mm256_loadu_si256(self.0.lines.as_ptr().add(i as usize) as *const __m256i);
                let l_2 = {
                    let mut result = _mm256_loadu_si256(stamp.0.lines.as_ptr().add(i as usize) as *const __m256i);

                    result = if x > 0 {
                        _mm256_slli_epi64(result,  x)
                    } else {
                        _mm256_srli_epi64(result, -x)
                    };

                    result
                };

                _mm256_storeu_si256(self.0.lines.as_ptr().add(i as usize) as *mut __m256i, _mm256_xor_si256(l_1, l_2));
            }
        }

        self.0.lines[ self.0.height      as usize] = 0;
        self.0.lines[(self.0.height + 1) as usize] = 0;
        self.0.lines[(self.0.height + 2) as usize] = 0;
    }

    pub fn stamp_on_top(&mut self, stamp: &Stamp, x: i32) {
        if self.0.width != 64 {
            self.stamp_on_top_with_mask(stamp, x);
        } else {
            self.stamp_on_top_without_mask(stamp, x);
        }
    }
}

#[derive(Clone)]
pub struct Field {
    width: i32,
    field_units: Vec<FieldUnit>
}

impl Field {
    pub fn new(field_units: Vec<FieldUnit>, width: i32) -> Field {
        Field {
            width,
            field_units
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.field_units[0].height()
    }

    pub fn field_units(&self) -> &[FieldUnit] {
        &self.field_units
    }

    pub fn count(&self) -> i32 {
        let mut result = 0;

        for field_unit in &self.field_units {
            result += field_unit.count();
        }

        result
    }

    pub fn stamp(&mut self, stamp: &Stamp, x: i32, y: i32) {
        let i_1 =  x                      / 64;
        let i_2 = (x + stamp.width() - 1) / 64;

        self.field_units[i_1 as usize].stamp(stamp, x % 64, y);

        if i_1 == i_2 || i_2 as usize == self.field_units.len() {
            return
        }

        self.field_units[i_2 as usize].stamp(stamp, x % 64 - 64, y);
    }

    pub fn stamp_on_top(&mut self, stamp: &Stamp, x: i32) {
        let i_1 =  x                      / 64;
        let i_2 = (x + stamp.width() - 1) / 64;

        self.field_units[i_1 as usize].stamp_on_top(stamp, x % 64);

        if i_1 == i_2 || i_2 as usize == self.field_units.len() {
            return
        }

        self.field_units[i_2 as usize].stamp_on_top(stamp, x % 64 - 64);
    }
}
