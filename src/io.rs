use std::arch::x86_64::*;
use std::cmp::*;
use std::io::*;
use std::str::*;

use super::game::*;

fn read_field(width: i32, height: i32, bits_str: &str) -> Field {
    let field_units = {
        let lines_collection = {
            let mut result = (0..width).step_by(64).map(|_| Vec::with_capacity(height as usize)).collect::<Vec<Vec<u64>>>();

            bits_str.as_bytes().chunks(width as usize).map(|bits_str| {
                bits_str.chunks(64).map(|bits_str| {
                    bits_str.iter().enumerate().filter(|(_, c)| **c as char == '1').fold(0, |acc, (x, _)| acc | 1 << x)
                })
            }).for_each(|bits_collection| {
                bits_collection.enumerate().for_each(|(i, bits)| {
                    result[i].push(bits);
                });
            });

            result
        };

        (0..width).step_by(64).map(|x| FieldUnit::new(lines_collection[(x / 64) as usize].clone(), min(width - x, 64))).collect()
    };

    Field::new(field_units, width)
}

fn read_stamp(width: i32, height: i32, bits_str: &str) -> (Stamp, (i32, i32)) {
    unsafe {
        let (lines, width, offset) = {
            let lines = {
                bits_str.as_bytes().chunks(width as usize).map(|bits_str| {
                    bits_str.iter().enumerate().filter(|(_, c)| **c as char == '1').fold(0, |acc, (x, _)| acc | 1 << x)
                }).collect::<Vec<u64>>()
            };

            let y_1 =          lines.iter().      take_while(|line| **line == 0).count() as i32;
            let y_2 = height - lines.iter().rev().take_while(|line| **line == 0).count() as i32;

            let (x_1, x_2) = {
                let mut x_1 = width;
                let mut x_2 = 0;

                for line in &lines {
                    x_1 = min(x_1,      _tzcnt_u64(*line) as i32);
                    x_2 = max(x_2, 64 - _lzcnt_u64(*line) as i32);
                }

                (x_1, x_2)
            };

            ((y_1..y_2).map(|y| lines[y as usize] >> x_1).collect::<Vec<_>>(), x_2 - x_1, (x_1, y_1))
        };

        (Stamp::new(lines, width), offset)
    }
}

pub fn read_question() -> (Field, Vec<Stamp>, Vec<(i32, i32)>) {
    let stdin = stdin();

    let handle = stdin.lock();
    let mut reader = BufReader::new(handle);

    let mut line = String::with_capacity(3 + 1 + 3 + 1 + 640 * 480 + 1);

    let field = {
        reader.read_line(&mut line).unwrap();
        let mut it = line.trim().split(';');

        let result = read_field(i32::from_str(it.next().unwrap()).unwrap(), i32::from_str(it.next().unwrap()).unwrap(), it.next().unwrap());

        line.clear();

        result
    };

    let (stamps, offsets) = {
        let mut stamps  = Vec::new();
        let mut offsets = Vec::new();

        while reader.read_line(&mut line).unwrap() > 0 {
            let mut it = line.trim().split(';');

            let (stamp, offset) = read_stamp(i32::from_str(it.next().unwrap()).unwrap(), i32::from_str(it.next().unwrap()).unwrap(), it.next().unwrap());

            stamps.push(stamp);
            offsets.push(offset);

            line.clear();
        }

        (stamps, offsets)
    };

    (field, stamps, offsets)
}

pub fn write_answer(answer: &[(i32, i32, i32)], offsets: &[(i32, i32)]) {
    let stdout = stdout();

    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);

    writeln!(writer, "{}", answer.len()).unwrap();

    for (i, x, y) in answer {
        let (offset_x, offset_y) = offsets[*i as usize];

        writeln!(writer, "{};{},{}", *i, *x - offset_x, *y - offset_y).unwrap();
    }
}
