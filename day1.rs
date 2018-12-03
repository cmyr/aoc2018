fn main() {
    test_one();
    // test_two();

    let input = include_str!("day1_input.txt");
    let p1 = input
        .lines()
        .map(|s| (s.as_bytes()[0], s[1..].parse::<isize>().unwrap()))
        .fold(
            0isize,
            |acc, (op, val)| if op == b'+' { acc + val } else { acc - val },
        );
    println!("part one: {}", p1);

    let vals = input
        .lines()
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    // println!("vals: {:?}", &vals);

    println!("part two: {}", first_dup(&vals));
}

fn first_dup(inputs: &[isize]) -> isize {
    let mut seen = ::std::collections::HashSet::new();
    let mut acc = 0isize;
    seen.insert(acc);
    loop {
        for val in inputs {
            acc += val;
            if !seen.insert(acc) {
                return acc;
            }
        }
    }
}

fn test_one() {
    assert_eq!(first_dup(&[1, -1]), 0);
    assert_eq!(first_dup(&[3, 3, 4, -2, -4]), 10);
    assert_eq!(first_dup(&[-6, 3, 8, 5, -6]), 5);
    assert_eq!(first_dup(&[7, 7, -2, -7, -4]), 14);
}
