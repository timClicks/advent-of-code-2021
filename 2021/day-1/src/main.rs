use std::iter::Iterator;


fn how_many_above(readings: &Vec<i32>) -> i32 {
    readings
        .windows(2)
        .map(|window| {
            let mut win = window.iter();
            let a = win.next().unwrap_or(&i32::MIN);
            let b = win.next().unwrap_or(&i32::MAX);

            // dbg!(a, b, window);

            if b > a {
                1
            } else {
                0
            }
        })
        .sum()
}

fn threes(readings: &Vec<i32>) -> Vec<i32> {
    readings
        .windows(3)
        .map(|window| {
            window.iter().sum()    
        })
        .collect()     
}


fn main() {
    let input: Vec<_> = include_str!("input.txt")
    .lines()
    .map(|line| line.trim())
    .map(|line| i32::from_str_radix(line, 10).unwrap())
    .collect();

    println!("{}", how_many_above(&threes(&input)) );
}
