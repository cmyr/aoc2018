use std::str::FromStr;

fn main() {
    test_parse();
    test_p1();
    let input = include_str!("input/day3.txt");
    let claims: Vec<Claim> = input.lines().map(|s| s.parse().unwrap()).collect();
    eprintln!("part one: {}", part_one(&claims));
    eprintln!("part two: {}", part_two(&claims));
}

fn part_one(claims: &[Claim]) -> usize {
    let max_y = claims.iter().map(|c| (c.y + c.h)).max().unwrap();
    let max_x = claims.iter().map(|c| (c.x + c.w)).max().unwrap();

    let mut squares: Vec<u8> = vec![0; max_y * max_x];
    for c in claims.iter() {
        for row in c.y..c.y + c.h {
            for col in c.x..c.x + c.w {
                let ix = row * max_x + col;
                squares[ix] = squares[ix].saturating_add(1);
            }
        }
    }
    squares.iter().filter(|i| **i >= 2).count()
}

fn part_two(claims: &[Claim]) -> usize {
    let max_y = claims.iter().map(|c| (c.y + c.h)).max().unwrap();
    let max_x = claims.iter().map(|c| (c.x + c.w)).max().unwrap();

    let mut squares: Vec<usize> = vec![0; max_y * max_x];
    for c in claims.iter() {
        for row in c.y..c.y + c.h {
            for col in c.x..c.x + c.w {
                let ix = row * max_x + col;
                let new_val = if squares[ix] != 0 {
                    usize::max_value()
                } else {
                    c.id
                };
                squares[ix] = new_val;
            }
        }
    }

    'iter: for c in claims.iter() {
        for row in c.y..c.y + c.h {
            for col in c.x..c.x + c.w {
                let ix = row * max_x + col;
                if squares[ix] != c.id {
                    continue 'iter;
                }
            }
        }
        // eprintln!("{}", c.id);
        return c.id;
    }
    usize::max_value()
}

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut iter = s.split_whitespace();
        let id = iter.next().unwrap()[1..].parse().unwrap();
        iter.next(); // skip @
        let mut xy = iter.next().unwrap().split(',');
        let x = xy.next().unwrap().parse().unwrap();
        let y = xy.next().unwrap().trim_matches(':').parse().unwrap();
        let mut size = iter.next().unwrap().split('x');
        let w = size.next().unwrap().parse().unwrap();
        let h = size.next().unwrap().parse().unwrap();
        Ok(Claim { id, x, y, w, h })
    }
}

fn test_parse() {
    let claims = &["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
    let Claim { id, x, y, w, h } = claims[0].parse().unwrap();
    assert_eq!((id, x, y, w, h), (1, 1, 3, 4, 4));
    let Claim { id, x, y, w, h } = claims[1].parse().unwrap();
    assert_eq!((id, x, y, w, h), (2, 3, 1, 4, 4));
    let Claim { id, x, y, w, h } = claims[2].parse().unwrap();
    assert_eq!((id, x, y, w, h), (3, 5, 5, 2, 2));
}

fn test_p1() {
    let claims = &["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
    let claims: Vec<Claim> = claims.iter().map(|s| s.parse().unwrap()).collect();
    assert_eq!(part_one(&claims), 4);
}
