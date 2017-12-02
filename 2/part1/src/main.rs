use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Reading line from stdin");

    let sum : u32 = input.lines()
        .map(|line| line.split_whitespace().map(|e| e.parse()).collect::<Vec<_>>())
        .map(|result_vec| result_vec.into_iter().map(|res| res.expect("a number")))
        .map(|nums| {
            let (min, max) = nums.fold((u32::max_value(), 0), |acc, x| ((acc.0).min(x), (acc.1).max(x)));
            println!("{} {}", min, max);
            max - min
        })
        .fold(0, |acc, x| acc + x);

    println!("The answer is: {}", sum);
}
