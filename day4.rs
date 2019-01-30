use std::str::FromStr;

fn main() {
    test_date_parse();
    let input = include_str!("input/day4.txt");
    let events: Vec<Entry> = input.lines()
    .map(|l| l.parse::<Entry>().unwrap())
    .collect();

    print!("{}", input);
    println!("{:?}", events);

}

#[derive(Debug, PartialEq, Eq)]
struct Datetime {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl FromStr for Datetime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches('[');
        let s = s.replace('-', " ");
        let s = s.replace(':', " ");
        let mut items = s.split_whitespace();

        let year = items.next().unwrap().parse::<usize>().unwrap();
        let month = items.next().unwrap().parse::<usize>().unwrap();
        let day = items.next().unwrap().parse::<usize>().unwrap();
        let hour = items.next().unwrap().parse::<usize>().unwrap();
        let minute = items.next().unwrap().parse::<usize>().unwrap();
        
        Ok(Datetime {
            year,
            month,
            day,
            hour,
            minute
        })
    }
}

fn test_date_parse() {
    let inp = "[1518-10-26 00:44";
    let result = inp.parse::<Datetime>().expect("parse failed!??!?");
    let expected = Datetime {
        year: 1518,
        month: 10,
        day: 26,
        hour: 0,
        minute: 44,
    };
    assert_eq!(result, expected);
    println!("success: {:?}", result);
}


#[derive(Debug)]
struct Entry {
    timestamp: Datetime,
    event: Event,
}

impl FromStr for Entry {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(']');
        let time = words.next().unwrap();
        let event = words.next().unwrap();
        let timestamp = time.parse::<Datetime>()?;
        let event = event.parse::<Event>()?;
        Ok( Entry { timestamp, event } )
    }
}

type GuardId = usize;

#[derive(Debug)]
enum Event {
    StartShift(GuardId),
    Sleep,
    Wake,
}

impl FromStr for Event {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s.trim() {
            "wakes up" => Event::Wake,
            "falls asleep" => Event::Sleep,
            other => {
                let num_str = other.trim_start_matches("Guard #")
                    .trim_end_matches(" begins shift");
                let guard_num = num_str.parse::<usize>().unwrap();
                Event::StartShift(guard_num)
            },
        };
        Ok(result)
    }
}
    