struct PasswordPolicy {
    letter: char,
    min: u8,
    max: u8,
}

impl PasswordPolicy {
    fn new(policy_str: &str) -> Self {
        let mut split = policy_str.split(" ");
        let minmax = split.next().unwrap();
        let letter = split.next().unwrap().chars().next().unwrap();
        PasswordPolicy {
            letter,
            min: 0,
            max: 0,
        }
    }
}

fn main() {
    let input = include_str!("input.txt")
    .lines()
    .map(|l| {
        let split = l.splitn(1,":");
        
    });
    println!("Hello, world!");
}
