use day_1::Rotator;

fn main() {
    let input = include_str!("../input.txt");
    let mut rotator = Rotator::default();

    let mut zero_counter = 0;

    for line in input.lines() {
        match rotator.parse_line(line.parse().unwrap()) {
            Ok(number) => {
                if number == 0 { zero_counter += 1}
            },
            Err(error) => {println!("error: {}", error);}
        }
    }

    println!("zero counter: {zero_counter}");
    println!("Zero occurrence counter {}", rotator.special_counter());
}
