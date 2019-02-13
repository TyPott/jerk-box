use std::env;

#[inline]
fn exaggerate(s: &str) -> String {
    s.chars().fold(String::new(), |s, c| s + &format!("{} ", c))
}

fn main() {
    let jerk = env::args().nth(1)
        .unwrap_or_else(|| String::from("lol no input"))
        .to_uppercase();

    let space = " ".repeat(jerk.len() * 2 - 3);

    println!("{}", exaggerate(&jerk));
    jerk.chars()
        .zip(jerk.chars().rev())
        .take(jerk.len() - 1)
        .skip(1)
        .for_each(|(i, j)| println!("{}{}{}", i, space, j));
    println!("{}", exaggerate(&jerk.chars().rev().collect::<String>()));
}
