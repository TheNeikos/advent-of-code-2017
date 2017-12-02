use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Reading line from stdin");
    input = input.trim().into();

    let mut repeat = input.chars().cycle();
    repeat.next();

    let mut sum = 0;
    for (cur, next) in input.chars().zip(repeat) {
        let a = cur.to_digit(10).expect(&format!("Could not parse {} as an u8", cur));
        let b = next.to_digit(10).expect(&format!("Could not parse {} as an u8", cur));

        if a == b {
            sum += a;
        }
    }

    println!("The answer is: {}", sum);
}
