const INPUT: &str = include_str!("input/day2.txt");

fn main() {
    let p1 = part_one(INPUT);
    println!("part one: {}", p1);

    let p2 = part_two(INPUT);
    println!("part two: {}", p2);
}

fn part_one(input: &str) -> usize {
    // let mut counts = vec![0; 256];
    let mut exact_twos = 0;
    let mut exact_threes = 0;

    for line in input.lines() {
        let mut counts = [0; 256];
        for b in line.as_bytes() {
            counts[*b as usize] += 1;
        }

        let mut has_two = false;
        let mut has_three = false;

        for i in 0..256 {
            if counts[i] == 2 {
                has_two = true;
            }
            if counts[i] == 3 {
                has_three = true;
            }
        }

        if has_two {
            exact_twos += 1;
        }
        if has_three {
            exact_threes += 1;
        }
    }
    exact_twos * exact_threes
}

fn part_two(input: &str) -> String {
    let all = input.lines().collect::<Vec<_>>();
    for (i, one) in all.iter().enumerate() {
        for (j, two) in all.iter().enumerate() {
            if i == j {
                continue;
            }
            if compare(one, two) == 1 {
                return common(one, two);
            }
        }
    }
    unreachable!()
}

fn compare(one: &str, two: &str) -> usize {
    assert_eq!(one.len(), two.len());
    one.as_bytes()
        .iter()
        .zip(two.as_bytes().iter())
        .filter(|(a, b)| a != b)
        .count()
}

fn common(one: &str, two: &str) -> String {
    one.chars()
        .zip(two.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect()
}
