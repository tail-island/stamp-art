use std::arch::x86_64::*;
use std::collections::*;

use super::game::*;

const W: i32   = 20;
const C: usize = 1 << W;
const M: u32   = 0xffff_ffff >> (32 - W);

#[inline(always)]
fn next_line_answer(prev_line_answer: &[(u8, u8)], s: u8, x: u8) -> Vec<(u8, u8)> {
    let mut result = Vec::with_capacity(prev_line_answer.len() + 1);

    result.extend_from_slice(prev_line_answer);
    result.push((s, x));

    result
}

#[inline(always)]
fn line_answers(stamp_lines: &[u32]) -> Vec<Vec<(u8, u8)>> {
    let mut result = Vec::new(); result.resize(C, Vec::new());

    let mut visited = Vec::<bool>::new(); visited.resize(C, false);
    let mut c = 1;

    let mut queue = VecDeque::with_capacity(C); queue.push_back(0);

    while c < C {
        let line = queue.pop_front().unwrap();

        for (s, stamp_line) in stamp_lines.iter().enumerate() {
            for x in 0..W {
                let next_line = line ^ stamp_line << x & M;

                if next_line != 0 && unsafe { !visited.get_unchecked(next_line as usize) } {
                    result[next_line as usize] = next_line_answer(&result[line as usize], s as u8, x as u8);

                    visited[next_line as usize] = true;
                    c += 1;

                    queue.push_back(next_line);
                }
            }
        }
    }

    result
}

#[inline(always)]
fn line(field: &Field, x: i32, y: i32) -> u32 {
    let i_1 =  x          / 64;
    let i_2 = (x + W - 1) / 64;

    let line_u64 = if i_1 == i_2 || i_2 as usize == field.field_units().len() {
        field.field_units()[i_1 as usize].lines()[y as usize] >>       x % 64
    } else {
        field.field_units()[i_1 as usize].lines()[y as usize] >>       x % 64 |
        field.field_units()[i_2 as usize].lines()[y as usize] << (64 - x % 64)
    };

    line_u64 as u32 & M
}

pub fn answer(mut field: Field, stamps: &[Stamp]) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();

    let offset_xs = stamps.iter().map(|stamp| unsafe { _tzcnt_u64(stamp.lines()[0]) } as i32).collect::<Vec<_>>();
    let line_answers = line_answers(&stamps.iter().zip(offset_xs.iter()).map(|(stamp, offset_x)| (stamp.lines()[0] >> offset_x) as u32).collect::<Vec<_>>());

    for y in 0..field.height() {
        for x_step in (0..field.width()).step_by(W as usize) {
            for (s, x_w) in &line_answers[line(&field, x_step, y) as usize] {
                let x = x_step + *x_w as i32 - offset_xs[*s as usize];

                if x < field.width() {
                    field.stamp(&stamps[*s as usize], x, y);

                    result.push((*s as i32, x, y));
                }
            }
        }
    }

    result
}
