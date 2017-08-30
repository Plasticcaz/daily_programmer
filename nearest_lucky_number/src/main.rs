fn main() {
    let start = std::time::Instant::now();

    find_lucky(103);
    find_lucky(225);
    find_lucky(997);

    let lucky_numbers = gen_lucky_nums(1_000_000);
    let time = start.elapsed();
    println!("{:?}", lucky_numbers);
    print!("Total time: ");
    print_duration(time);
    println!();
}

fn find_lucky(n: usize) {
    let num_to_gen = n + (0.1 * (n as f32) ) as usize;
    let lucky_numbers = gen_lucky_nums(num_to_gen);
    let mut i = 0;
    while lucky_numbers[i] < n {
        i += 1;
    }
    
    if lucky_numbers[i] == n {
        println!("{} is a lucky number", n);
    }
    else {
        println!("{} < {} < {}", lucky_numbers[i-1], n, lucky_numbers[i]);
    }
}

/// Generate all the "lucky" numbers from 1 to n.
fn gen_lucky_nums(n: usize) -> Vec<usize> {
    // Initialize with all odd numbers
    let mut numbers = Vec::with_capacity(n/2);
    for i in 0..n {
        if i % 2 != 0 {
            numbers.push(i)
        }
    }
    // Now skip each 
    let mut skip_index = 1;
    let mut to_remove = Vec::with_capacity((numbers.len()/ 3) + 1);
    while skip_index < numbers.len() {
        let to_skip = numbers[skip_index] - 1;
        let mut i = to_skip;
        while i < numbers.len() {
            to_remove.push(i);
            i += to_skip + 1;
        }
        // Remove all numbers.
        for i in to_remove.iter().rev() {
            numbers.remove(*i);
        }
        to_remove.clear();
        skip_index += 1;
    }

    numbers
}

/// Print the duration as seconds in decimal.
fn print_duration(duration: std::time::Duration) {
    let secs = duration.as_secs() as f64;
    let nanos = duration.subsec_nanos() as f64;
    let seconds = secs + (nanos / 1_000_000_000.0);
    print!("{}", seconds);
}