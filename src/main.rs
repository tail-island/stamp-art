use std::time::*;

use stamp_art::beam_solver;
use stamp_art::bfs_solver;
use stamp_art::hill_climbing_solver;
use stamp_art::io::*;

fn main() {
    let instant  = Instant::now();
    let duration = Duration::from_millis(9000);

    let (field, stamps, offsets) = read_question();

    let answer = {
        let mut result = bfs_solver::answer(field.clone(), &stamps);
        eprintln!("{}\t{}", result.len(), instant.elapsed().as_millis());

        let answers = [1, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192].iter().map(|beam_width| {
            if *beam_width == 1 {
                hill_climbing_solver::answer(field.clone(), &stamps, &instant, &duration)
            } else {
                beam_solver::answer(field.clone(), &stamps, *beam_width, &instant, &duration)
            }
        }).take_while(|option| option.is_some()).map(|option| option.unwrap());

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
