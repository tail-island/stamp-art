use rayon::*;
use rayon::prelude::*;
use std::time::*;

use stamp_art::beam_solver;
use stamp_art::bfs_solver;
use stamp_art::hill_climbing_solver;
use stamp_art::io::*;

const DURATION: Duration = Duration::from_millis(9000);

fn main() {
    let instant = Instant::now();

    ThreadPoolBuilder::new().num_threads(2).build_global().unwrap();

    let (field, stamps, offsets) = read_question();

    let answer = {
        let mut result = bfs_solver::answer(field.clone(), &stamps);
        eprintln!("{}\t{}", result.len(), instant.elapsed().as_millis());

        let answers = [1, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192].iter().map(|beam_width| {
            let answer = if *beam_width == 1 {
                hill_climbing_solver::answer(field.clone(), &stamps, &instant, &DURATION)
            } else {
                beam_solver::answer(field.clone(), &stamps, *beam_width, &instant, &DURATION)
            };

            if let Some(answer) = &answer {
                eprintln!("{}\t{}\t{}", answer.len(), instant.elapsed().as_millis(), beam_width);
            }

            answer
        }).collect::<Vec<_>>();

        for answer in answers {
            if let Some(answer) = answer {
                if answer.len() < result.len() {
                    result = answer;
                }
            }
        }

        result
    };

    write_answer(&answer, &offsets);
}
