use std::env;

fn parse_args(args: &mut env::Args) -> String {
    args.nth(1).unwrap_or_else(|| String::from("lol no input")).to_uppercase()
}

fn exaggerate(s: &str) -> String {
    s.chars().map(|c| format!("{} ", c)).collect()
}

fn main() {
    let jerk = parse_args(&mut env::args());
    let space: String = vec![' '; jerk.len() * 2 - 3].into_iter().collect();

    println!("{}", exaggerate(&jerk));
    jerk.chars()
        .zip(jerk.chars().rev())
        .skip(1)
        .take(jerk.len() - 2)
        .for_each(|(i, j)| println!("{}{}{}", i, space, j));
    println!("{}", exaggerate(&jerk.chars().rev().collect::<String>()));
}
