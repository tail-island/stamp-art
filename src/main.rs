use std::time::*;

use stamp_art::beam_solver;
use stamp_art::breadth_first_solver;
use stamp_art::greedy_solver;
use stamp_art::io::*;

fn main() {
    let instant  = Instant::now();
    let duration = Duration::from_millis(9000);

    let (field, stamps, offsets) = read_question();

    let answer = {
        let (mut result, field, stamps, is_rev) = {
            let result = breadth_first_solver::answer(field.clone(), &stamps);
            eprintln!("{}\t{}", result.len(), instant.elapsed().as_millis());

            let (rev_field, rev_stamps) = rev_question(&field, &stamps);

            let rev_result = breadth_first_solver::answer(rev_field.clone(), &rev_stamps);
            eprintln!("{}\t{}", rev_result.len(), instant.elapsed().as_millis());

            if result.len() <= rev_result.len() {
                (result, field, stamps, false)
            } else {
                (rev_result, rev_field, rev_stamps, true)
            }
        };

        for beam_width in &[1, 32, 128, 256, 512, 1024, 2048, 4096] {
            let answer = if *beam_width == 1 {
                greedy_solver::answer(field.clone(), &stamps, &instant, &duration)
            } else {
                beam_solver::answer(field.clone(), &stamps, *beam_width, &instant, &duration)
            };

            if let Some(answer) = answer {
                eprintln!("{}\t{}", answer.len(), instant.elapsed().as_millis());

                if answer.len() < result.len() {
                    result = answer;
                }
            } else {
                break;
            }
        }

        if is_rev {
            result = result.into_iter().map(|(s, x, y)| (s, x, field.height() - y - stamps[s as usize].height())).collect::<Vec<_>>();
        }

        result
    };

    write_answer(&answer, &offsets);

    eprintln!("{}\t{} !", answer.len(), instant.elapsed().as_millis());
}
