#![allow(dead_code)]

#[derive(Debug)]
struct PasswordPolicy {
    letter: char,
    x: u8,
    y: u8,
}

impl PasswordPolicy {
    fn new(policy: &str) -> Self {
        let (minmax, letter) = split2(policy, " ");
        let (min, max) = split2(minmax, "-");
        PasswordPolicy {
            letter: letter.chars().next().unwrap(),
            x: min.parse().unwrap(),
            y: max.parse().unwrap(),
        }
    }
    fn check1(&self, pw: &str) -> bool {
        let count = pw.chars().filter(|c| c == &self.letter).count();
        count >= self.x as usize && count <= self.y as usize
    }
    fn check2(&self, pw: &str) -> bool {
        let mut chars = pw.chars();
        let x = chars.nth(self.x as usize - 1).unwrap_or('_');
        let y = chars.nth((self.y - self.x) as usize - 1).unwrap_or('_');
        (x == self.letter && y != self.letter) || (x != self.letter && y == self.letter)
    }
}

fn split2<'a>(input: &'a str, pat: &str) -> (&'a str, &'a str) {
    let mut split = input.splitn(2, pat);
    (split.next().unwrap(), split.next().unwrap())
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .filter(|l| {
            let (pwp, pw) = split2(l, ": ");
            PasswordPolicy::new(pwp).check2(pw)
        })
        .count();
    println!("{:#?}", input);
}
