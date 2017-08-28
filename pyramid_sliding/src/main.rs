mod pyramid;
use pyramid::*;

fn main() {
    let filename = match std::env::args().nth(1) {
        Some(s) => s,
        None => {
            println!("Please provide a file as an argument.");
            return;
        }
    };

    match pyramid::load_pyramid(&filename) {
        Ok(pyramid) => {
            let start = std::time::Instant::now();
            let result = slide_down_greedily(&pyramid);
            let run_time = start.elapsed();
            print!("result: {}, elapsed time: ", result);
            print_duration(run_time);
            println!();

        }
        Err(msg) => println!("{}", msg),
    }
}

/// Search greedily for the least-cost route down the pyramid, returning the total
/// cost. NOTE: This method is NOT guarenteed to find the best path.
fn slide_down_greedily(pyramid: &Pyramid) -> usize {
    // Somewhat confusingly, level 0 is the top level... just roll with it.
    let mut current = Location { level: 0, block: 0};
    let mut total_cost = pyramid.data[0];
    for _ in 0..pyramid.height-1 {
        // Look at the choices, and pick the best one, greedily.
        let right_choice = right_choice(&current);
        let left_choice = left_choice(&current);
        let right_cost = cost_of(pyramid, &right_choice);
        let left_cost = cost_of(pyramid, &left_choice);

        if right_cost < left_cost {
            current = right_choice;
            total_cost += right_cost;
        } else {
            current = left_choice;
            total_cost += left_cost
        }
    }

    total_cost
}

/// Print the duration as seconds in decimal.
fn print_duration(duration: std::time::Duration) {
    let secs = duration.as_secs() as f64;
    let nanos = duration.subsec_nanos() as f64;
    let seconds = secs + (nanos / 1_000_000_000.0);
    print!("{}", seconds);
}