use std::arch::x86_64::*;
use std::collections::*;
use std::cmp::*;
use std::time::*;

use super::game::*;

fn target_field(field: &Field, stamps: &[Stamp], y: i32) -> Field {
    let height = min(stamps.iter().map(|stamp| stamp.height()).max().unwrap(), field.height() - y);

    Field::new(
        field.field_units().iter().map(|field_unit| FieldUnit::new(field_unit.lines()[y as usize..(y + height) as usize].to_vec(), field_unit.width())).collect(),
        field.width()
    )
}

fn target_x(field: &Field) -> Option<i32> {
    field.field_units().iter().enumerate().find_map(|(i, field_unit)| {
        if field_unit.lines()[0] != 0 {
            Some(i as i32 * 64 + unsafe { _tzcnt_u64(field_unit.lines()[0]) } as i32)
        } else {
            None
        }
    })
}

fn next_field(prev_field: &Field, stamp: &Stamp, x: i32) -> Field {
    let mut result = prev_field.clone();

    result.stamp_on_top(stamp, x);

    result
}

fn next_line_answer(prev_line_answer: &[(i16, i16)], s: i16, x: i16) -> Vec<(i16, i16)> {
    let mut result = Vec::with_capacity(prev_line_answer.len() + 1);

    result.extend_from_slice(prev_line_answer);
    result.push((s, x));

    result
}

fn next_score(next_field: &Field) -> i32 {
    -(next_field.field_units().iter().map(|field_unit| unsafe { _popcnt64(field_unit.lines()[0] as i64) } as i32).sum::<i32>() * next_field.height() +
      next_field.count())
}

fn line_answer(field: Field, stamps: &[Stamp], beam_width: i32, instant: &Instant, duration: &Duration) -> Option<Vec<(i16, i16)>> {
    struct Node {
        field: Field,
        score: i32,
        line_answer: Vec<(i16, i16)>
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.score == other.score
        }
    }

    impl Eq for Node {}

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(&other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.score.cmp(&other.score)
        }
    }

    let mut prev_queue = BinaryHeap::with_capacity(1);

    prev_queue.push(Node {
        field,
        score: 0,
        line_answer: Vec::new()
    });

    while instant.elapsed() <= *duration {
        let mut next_queue = BinaryHeap::with_capacity(beam_width as usize * stamps.len());

        for _ in 0..min(beam_width, prev_queue.len() as i32) {
            let prev = prev_queue.pop().unwrap();

            if let Some(target_x) = target_x(&prev.field) {
                for (s, stamp) in stamps.iter().enumerate() {
                    let x = target_x - unsafe { _tzcnt_u64(stamp.lines()[0]) } as i32;

                    let next_field = next_field(&prev.field, stamp, x);
                    let next_score = next_score(&next_field);
                    let next_line_answer = next_line_answer(&prev.line_answer, s as i16, x as i16);

                    next_queue.push(Node {
                        field: next_field,
                        score: next_score,
                        line_answer: next_line_answer
                    });
                }
            } else {
                return Some(prev.line_answer);
            }
        }

        prev_queue = next_queue;
    }

    None
}

pub fn answer(mut field: Field, stamps: &[Stamp], beam_width: i32, instant: &Instant, duration: &Duration) -> Option<Vec<(i32, i32, i32)>> {
    let mut result = Vec::new();

    for y in 0..field.height() {
        if let Some(line_answer) = line_answer(target_field(&field, stamps, y), stamps, beam_width, instant, duration) {
            for (s, x) in &line_answer {
                field.stamp(&stamps[*s as usize], *x as i32, y);

                result.push((*s as i32, *x as i32, y));
            }
        } else {
            return None;
        }
    }

    Some(result)
}
