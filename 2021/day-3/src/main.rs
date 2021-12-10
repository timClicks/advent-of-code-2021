use std::collections::HashMap;

fn diagnose(input: &str) -> f32 {
    let mut bit_counts = vec![];

    for line in input.lines() {
        bit_counts.resize_with(line.len(), HashMap::new);

        for (position, byte) in line.chars().enumerate() {
            let map = &mut bit_counts[position];
            let k = map.entry(byte).or_insert(0);
            *k += 1;
        }
    }

    let mut gamma = String::with_capacity(bit_counts.len());
    let mut epsilon = String::with_capacity(bit_counts.len());

    for map in bit_counts {
        if map[&'0'] > map[&'1'] {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma = i32::from_str_radix(&gamma, 2).unwrap() as f32;
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap() as f32;

    gamma * epsilon
}

fn main() {
   let input = include_str!("input.txt");

    println!("{}", diagnose(input));
}
