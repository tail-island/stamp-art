use std::arch::x86_64::*;
use std::cmp::*;
use std::time::*;

use super::game::*;

fn target_x(field: &Field, y: i32) -> Option<i32> {
    field.field_units().iter().enumerate().find_map(|(i, field_unit)| {
        if field_unit.lines()[y as usize] != 0 {
            Some(i as i32 * 64 + unsafe { _tzcnt_u64(field_unit.lines()[y as usize]) } as i32)
        } else {
            None
        }
    })
}

pub fn answer(mut field: Field, stamps: &[Stamp], instant: &Instant, duration: &Duration) -> Option<Vec<(i32, i32, i32)>> {
    let mut result = Vec::new();

    let stamp_max_height = stamps.iter().map(|stamp| stamp.height()).max().unwrap();

    for y in 0..field.height() {
        if instant.elapsed() <= *duration {
            while let Some(target_x) = target_x(&field, y) {
                let mut best_cost = i32::max_value();
                let mut best_s = 0;
                let mut best_x = 0;

                for (s, stamp) in stamps.iter().enumerate() {
                    let x = target_x - unsafe { _tzcnt_u64(*stamp.lines().get_unchecked(0)) } as i32;

                    field.stamp(stamp, x, y);

                    let cost = field.field_units().iter().map(|field_unit| popcount_u64s(&field_unit.lines()[y as usize..(y + (min(stamp_max_height, field.height() - y) + 3) & !0b0011) as usize])).sum();

                    if cost < best_cost {
                        best_cost = cost;
                        best_s = s;
                        best_x = x;
                    }

                    field.stamp(stamp, x, y);
                }

                field.stamp(&stamps[best_s as usize], best_x, y);
                result.push((best_s as i32, best_x as i32, y as i32));
            }
        } else {
            return None;
        }
    }

    Some(result)
}
