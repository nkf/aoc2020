fn main() {
    let input = include_str!("input.txt");

    let count = count_slope(input, 1, 1)
        * count_slope(input, 3, 1)
        * count_slope(input, 5, 1)
        * count_slope(input, 7, 1)
        * count_slope(input, 1, 2);
    println!("{}", count);
}

fn count_slope(slope: &str, right: usize, down: usize) -> usize {
    let mut x = 0;
    let mut trees = 0;
    for (i, line) in slope.lines().enumerate().skip(down) {
        if i % down != 0 {
            continue;
        }
        x = (x + right) % line.len();

        match line.chars().nth(x) {
            Some('#') => trees += 1,
            Some(_) => (),
            None => break,
        }
    }
    trees
}
