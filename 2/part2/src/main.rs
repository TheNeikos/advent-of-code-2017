use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Reading line from stdin");

    let sum : u32 = input.lines()
        .map(|line| line.split_whitespace().map(|e| e.parse()).collect::<Vec<_>>())
        .map(|result_vec| result_vec.into_iter().map(|res| res.expect("a number")).collect::<Vec<_>>())
        .map(|nums| {
            let mut alpha = 0;
            let mut beta = 0;

            'outer: for &num in &nums {
                for &num2 in &nums {
                    if num % num2 == 0 && num != num2 {
                        alpha = num;
                        beta = num2;
                        break 'outer;
                    }
                }
            }

            if alpha > beta { alpha / beta } else { beta / alpha }
        })
        .fold(0, |acc, x| acc + x);

    println!("The answer is: {}", sum);
}
