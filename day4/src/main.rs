#![allow(dead_code)]

use regex::Regex;

fn main() {
    let input = include_str!("input.txt").split("\n\n").map(|pas| {
        pas.split(char::is_whitespace)
            .filter(|s| !s.starts_with("cid") && !s.is_empty())
            .collect::<Vec<_>>()
    });
    part2(input);
}

fn part1(input: impl Iterator<Item = Vec<&'static str>>) {
    println!("{:#?}", input.filter(|ppv| ppv.len() == 7).count());
}

fn part2(input: impl Iterator<Item = Vec<&'static str>>) {
    let re = Regex::new(r"(byr:((19[2-9][0-9])|(200[0-2])))|(iyr:((201[0-9])|(2020)))|(eyr:((202[0-9])|(2030)))|(hgt:((1[5-8][0-9]cm)|(19[0-3]cm)|(59in)|(6[0-9]in)|(7[0-6]in)))|(hcl:#[a-f0-9]{6})|(ecl:(amb|blu|brn|gry|grn|hzl|oth))|(pid:[0-9]{9}$)").unwrap();
    let x = input
        .filter(|ppv| {
            if ppv.len() != 7 {
                return false;
            }
            for pp in ppv {
                if !re.is_match(pp) {
                    println!("{}", pp);
                    return false;
                }
            }
            true
        })
        .count();

    println!("{}", x)
}
