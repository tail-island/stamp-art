use std::arch::x86_64::*;
use std::cmp::*;

// なんでだか_mm256_slli_epi64と_mm256_srli_epi64のimm8がconst genericsになっていた。。。

unsafe fn _mm256_slli_epi64_(a: __m256i, imm8: i32) -> __m256i {
    match imm8 {
         0 => _mm256_slli_epi64::< 0>(a),
         1 => _mm256_slli_epi64::< 1>(a),
         2 => _mm256_slli_epi64::< 2>(a),
         3 => _mm256_slli_epi64::< 3>(a),
         4 => _mm256_slli_epi64::< 4>(a),
         5 => _mm256_slli_epi64::< 5>(a),
         6 => _mm256_slli_epi64::< 6>(a),
         7 => _mm256_slli_epi64::< 7>(a),
         8 => _mm256_slli_epi64::< 8>(a),
         9 => _mm256_slli_epi64::< 9>(a),
        10 => _mm256_slli_epi64::<10>(a),
        11 => _mm256_slli_epi64::<11>(a),
        12 => _mm256_slli_epi64::<12>(a),
        13 => _mm256_slli_epi64::<13>(a),
        14 => _mm256_slli_epi64::<14>(a),
        15 => _mm256_slli_epi64::<15>(a),
        16 => _mm256_slli_epi64::<16>(a),
        17 => _mm256_slli_epi64::<17>(a),
        18 => _mm256_slli_epi64::<18>(a),
        19 => _mm256_slli_epi64::<19>(a),
        20 => _mm256_slli_epi64::<20>(a),
        21 => _mm256_slli_epi64::<21>(a),
        22 => _mm256_slli_epi64::<22>(a),
        23 => _mm256_slli_epi64::<23>(a),
        24 => _mm256_slli_epi64::<24>(a),
        25 => _mm256_slli_epi64::<25>(a),
        26 => _mm256_slli_epi64::<26>(a),
        27 => _mm256_slli_epi64::<27>(a),
        28 => _mm256_slli_epi64::<28>(a),
        29 => _mm256_slli_epi64::<29>(a),
        30 => _mm256_slli_epi64::<30>(a),
        31 => _mm256_slli_epi64::<31>(a),
        32 => _mm256_slli_epi64::<32>(a),
        33 => _mm256_slli_epi64::<33>(a),
        34 => _mm256_slli_epi64::<34>(a),
        35 => _mm256_slli_epi64::<35>(a),
        36 => _mm256_slli_epi64::<36>(a),
        37 => _mm256_slli_epi64::<37>(a),
        38 => _mm256_slli_epi64::<38>(a),
        39 => _mm256_slli_epi64::<39>(a),
        40 => _mm256_slli_epi64::<40>(a),
        41 => _mm256_slli_epi64::<41>(a),
        42 => _mm256_slli_epi64::<42>(a),
        43 => _mm256_slli_epi64::<43>(a),
        44 => _mm256_slli_epi64::<44>(a),
        45 => _mm256_slli_epi64::<45>(a),
        46 => _mm256_slli_epi64::<46>(a),
        47 => _mm256_slli_epi64::<47>(a),
        48 => _mm256_slli_epi64::<48>(a),
        49 => _mm256_slli_epi64::<49>(a),
        50 => _mm256_slli_epi64::<50>(a),
        51 => _mm256_slli_epi64::<51>(a),
        52 => _mm256_slli_epi64::<52>(a),
        53 => _mm256_slli_epi64::<53>(a),
        54 => _mm256_slli_epi64::<54>(a),
        55 => _mm256_slli_epi64::<55>(a),
        56 => _mm256_slli_epi64::<56>(a),
        57 => _mm256_slli_epi64::<57>(a),
        58 => _mm256_slli_epi64::<58>(a),
        59 => _mm256_slli_epi64::<59>(a),
        60 => _mm256_slli_epi64::<60>(a),
        61 => _mm256_slli_epi64::<61>(a),
        62 => _mm256_slli_epi64::<62>(a),
        63 => _mm256_slli_epi64::<63>(a),
        64 => _mm256_slli_epi64::<64>(a),
        _ => panic!()
    }
}

unsafe fn _mm256_srli_epi64_(a: __m256i, imm8: i32) -> __m256i {
    match imm8 {
        0 => _mm256_srli_epi64::< 0>(a),
        1 => _mm256_srli_epi64::< 1>(a),
        2 => _mm256_srli_epi64::< 2>(a),
        3 => _mm256_srli_epi64::< 3>(a),
        4 => _mm256_srli_epi64::< 4>(a),
        5 => _mm256_srli_epi64::< 5>(a),
        6 => _mm256_srli_epi64::< 6>(a),
        7 => _mm256_srli_epi64::< 7>(a),
        8 => _mm256_srli_epi64::< 8>(a),
        9 => _mm256_srli_epi64::< 9>(a),
       10 => _mm256_srli_epi64::<10>(a),
       11 => _mm256_srli_epi64::<11>(a),
       12 => _mm256_srli_epi64::<12>(a),
       13 => _mm256_srli_epi64::<13>(a),
       14 => _mm256_srli_epi64::<14>(a),
       15 => _mm256_srli_epi64::<15>(a),
       16 => _mm256_srli_epi64::<16>(a),
       17 => _mm256_srli_epi64::<17>(a),
       18 => _mm256_srli_epi64::<18>(a),
       19 => _mm256_srli_epi64::<19>(a),
       20 => _mm256_srli_epi64::<20>(a),
       21 => _mm256_srli_epi64::<21>(a),
       22 => _mm256_srli_epi64::<22>(a),
       23 => _mm256_srli_epi64::<23>(a),
       24 => _mm256_srli_epi64::<24>(a),
       25 => _mm256_srli_epi64::<25>(a),
       26 => _mm256_srli_epi64::<26>(a),
       27 => _mm256_srli_epi64::<27>(a),
       28 => _mm256_srli_epi64::<28>(a),
       29 => _mm256_srli_epi64::<29>(a),
       30 => _mm256_srli_epi64::<30>(a),
       31 => _mm256_srli_epi64::<31>(a),
       32 => _mm256_srli_epi64::<32>(a),
       33 => _mm256_srli_epi64::<33>(a),
       34 => _mm256_srli_epi64::<34>(a),
       35 => _mm256_srli_epi64::<35>(a),
       36 => _mm256_srli_epi64::<36>(a),
       37 => _mm256_srli_epi64::<37>(a),
       38 => _mm256_srli_epi64::<38>(a),
       39 => _mm256_srli_epi64::<39>(a),
       40 => _mm256_srli_epi64::<40>(a),
       41 => _mm256_srli_epi64::<41>(a),
       42 => _mm256_srli_epi64::<42>(a),
       43 => _mm256_srli_epi64::<43>(a),
       44 => _mm256_srli_epi64::<44>(a),
       45 => _mm256_srli_epi64::<45>(a),
       46 => _mm256_srli_epi64::<46>(a),
       47 => _mm256_srli_epi64::<47>(a),
       48 => _mm256_srli_epi64::<48>(a),
       49 => _mm256_srli_epi64::<49>(a),
       50 => _mm256_srli_epi64::<50>(a),
       51 => _mm256_srli_epi64::<51>(a),
       52 => _mm256_srli_epi64::<52>(a),
       53 => _mm256_srli_epi64::<53>(a),
       54 => _mm256_srli_epi64::<54>(a),
       55 => _mm256_srli_epi64::<55>(a),
       56 => _mm256_srli_epi64::<56>(a),
       57 => _mm256_srli_epi64::<57>(a),
       58 => _mm256_srli_epi64::<58>(a),
       59 => _mm256_srli_epi64::<59>(a),
       60 => _mm256_srli_epi64::<60>(a),
       61 => _mm256_srli_epi64::<61>(a),
       62 => _mm256_srli_epi64::<62>(a),
       63 => _mm256_srli_epi64::<63>(a),
       64 => _mm256_srli_epi64::<64>(a),
       _ => panic!()
   }
}

#[inline(always)]
fn popcount_u64s(u64s: &[u64]) -> i32 {  // u64sの要素数は4の整数倍。
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
    #[inline(always)]
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

    #[inline(always)]
    fn count(&self) -> i32 {
        popcount_u64s(&self.lines[0..((self.height + 3) & !0b0011) as usize])
    }

    #[inline(always)]
    fn count_in(&self, y: i32, height: i32) -> i32 {
        popcount_u64s(&self.lines[y as usize..(y + (height + 3) & !0b0011) as usize])
    }
}

pub struct Stamp(Bitmap);

impl Stamp {
    #[inline(always)]
    pub fn new(lines: Vec<u64>, width: i32) -> Stamp {
        Stamp(Bitmap::new(lines, width))
    }

    #[inline(always)]
    pub fn width(&self) -> i32 {
        self.0.width
    }

    #[inline(always)]
    pub fn height(&self) -> i32 {
        self.0.height
    }

    #[inline(always)]
    pub fn lines(&self) -> &[u64] {
        &self.0.lines
    }

    #[inline(always)]
    pub fn count(&self) -> i32 {
        self.0.count()
    }
}

#[derive(Clone)]
pub struct FieldUnit(Bitmap);

impl FieldUnit {
    #[inline(always)]
    pub fn new(lines: Vec<u64>, width: i32) -> FieldUnit {
        FieldUnit(Bitmap::new(lines, width))
    }

    #[inline(always)]
    pub fn width(&self) -> i32 {
        self.0.width
    }

    #[inline(always)]
    pub fn height(&self) -> i32 {
        self.0.height
    }

    #[inline(always)]
    pub fn lines(&self) -> &[u64] {
        &self.0.lines
    }

    #[inline(always)]
    pub fn count(&self) -> i32 {
        self.0.count()
    }

    #[inline(always)]
    pub fn count_in(&self, y: i32, height: i32) -> i32 {
        self.0.count_in(y, height)
    }

    #[inline(always)]
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
                        _mm256_slli_epi64_(result,  x)
                    } else {
                        _mm256_srli_epi64_(result, -x)
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

    #[inline(always)]
    fn stamp_on_top_with_mask(&mut self, stamp: &Stamp, x: i32) {
        unsafe {
            let height = min(self.0.height, stamp.0.height);
            let m = _mm256_set1_epi64x(((0xffffffffffffffff as u64) >> (64 - self.0.width)) as i64);

            for i in (0..((height + 3) & !0b0011)).step_by(4) {
                let l_1 = _mm256_loadu_si256(self.0.lines.as_ptr().add(i as usize) as *const __m256i);
                let l_2 = {
                    let mut result = _mm256_loadu_si256(stamp.0.lines.as_ptr().add(i as usize) as *const __m256i);

                    result = if x > 0 {
                        _mm256_slli_epi64_(result,  x)
                    } else {
                        _mm256_srli_epi64_(result, -x)
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

    #[inline(always)]
    fn stamp_on_top_without_mask(&mut self, stamp: &Stamp, x: i32) {
        unsafe {
            let height = min(self.0.height, stamp.0.height);

            for i in (0..((height + 3) & !0b0011)).step_by(4) {
                let l_1 = _mm256_loadu_si256(self.0.lines.as_ptr().add(i as usize) as *const __m256i);
                let l_2 = {
                    let mut result = _mm256_loadu_si256(stamp.0.lines.as_ptr().add(i as usize) as *const __m256i);

                    result = if x > 0 {
                        _mm256_slli_epi64_(result,  x)
                    } else {
                        _mm256_srli_epi64_(result, -x)
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

    #[inline(always)]
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
    #[inline(always)]
    pub fn new(field_units: Vec<FieldUnit>, width: i32) -> Field {
        Field {
            width,
            field_units
        }
    }

    #[inline(always)]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> i32 {
        self.field_units[0].height()
    }

    #[inline(always)]
    pub fn field_units(&self) -> &[FieldUnit] {
        &self.field_units
    }

    #[inline(always)]
    pub fn count(&self) -> i32 {
        let mut result = 0;

        for field_unit in &self.field_units {
            result += field_unit.count();
        }

        result
    }

    #[inline(always)]
    pub fn count_in(&self, y: i32, height: i32) -> i32 {
        let mut result = 0;

        for field_unit in &self.field_units {
            result += field_unit.count_in(y, height);
        }

        result
    }

    #[inline(always)]
    pub fn stamp(&mut self, stamp: &Stamp, x: i32, y: i32) {
        let i_1 =  x                      / 64;
        let i_2 = (x + stamp.width() - 1) / 64;

        self.field_units[i_1 as usize].stamp(stamp, x % 64, y);

        if i_1 == i_2 || i_2 as usize == self.field_units.len() {
            return
        }

        self.field_units[i_2 as usize].stamp(stamp, x % 64 - 64, y);
    }

    #[inline(always)]
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
