use std::time::*;
use std::iter::*;

use stamp_art::beam_solver;
use stamp_art::bfs_solver;
use stamp_art::hill_climbing_solver;
use stamp_art::io::*;

const DURATION:    Duration = Duration::from_millis(9000);
const BEAM_WIDTHS: [i32; 8] = [32, 64, 128, 256, 512, 1024, 2048, 4096];

fn main() {
    let instant = Instant::now();

    let (field, stamps, offsets) = read_question();

    let answer = {
        let mut result = bfs_solver::answer(field.clone(), &stamps);
        eprintln!("{}\t{}", result.len(), instant.elapsed().as_millis());

        let answers = once(hill_climbing_solver::answer(field.clone(), &stamps, &instant, &DURATION)).chain(
            BEAM_WIDTHS.iter().map(|beam_width| beam_solver::answer(field.clone(), &stamps, *beam_width, &instant, &DURATION))
        ).take_while(|answer| answer.is_some()).map(|answer| answer.unwrap());

        for answer in answers {
            eprintln!("{}\t{}", answer.len(), instant.elapsed().as_millis());

            if answer.len() < result.len() {
                result = answer;
            }
        }

        result
    };

    write_answer(&answer, &offsets);
}
