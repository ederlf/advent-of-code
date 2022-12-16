#[derive(Debug)]
enum Position {
    Number(u32),
    List(Vec<Position>),
}

fn parse_packet(packet: &str) -> Vec<Position> {
    let mut p: Vec<Position> = Vec::new();
    let mut iter = packet.chars().peekable();
    while let Some(c) = iter.next() {
        if c == '[' {
            p.push(Position::List(Vec::new()));
        } else if c == ']' {
            let last = p.pop().unwrap();
            if p.len() > 0 {
                match p.last_mut().unwrap() {
                    Position::List(v) => v.push(last),
                    _ => (),
                };
            } else if let Position::List(v) = last {
                return v;
            }
        } else if c.is_digit(10) {
            let mut num = c.to_digit(10).unwrap();
            let next = iter.peek();
            if next != None {
                let next_digit = next.unwrap();
                if next_digit.is_digit(10) {
                    num = format!("{}{}", c, next_digit).parse::<u32>().unwrap();
                    iter.next();
                }
            }
            match p.last_mut().unwrap() {
                Position::List(v) => v.push(Position::Number(num)),
                _ => (),
            };
        }
    }
    p
}

fn compare_packets(p1: &Vec<Position>, p2: &Vec<Position>) -> (bool, bool) {
    for (x, y) in p1.iter().zip(p2) {
        let (cont, res) = match (x, y) {
            (Position::Number(v1), Position::Number(v2)) => {
                if *v1 > *v2 {
                    return (false, false);
                } else if *v1 < *v2 {
                    return (false, true);
                }
                (true, true)
            }
            (Position::List(v1), Position::List(v2)) => compare_packets(v1, v2),
            (Position::List(v1), Position::Number(v2)) => {
                let mut nv = Vec::new();
                nv.push(Position::Number(*v2));
                compare_packets(v1, &nv)
            }
            (Position::Number(v1), Position::List(v2)) => {
                let mut nv = Vec::new();
                nv.push(Position::Number(*v1));
                compare_packets(&nv, v2)
            }
        };

        if !cont {
            return (false, res);
        }
    }

    if p1.is_empty() {
        return (false, true);
    }

    // if all the comparisons on matching positions are the same
    // we get here. If p1 is larger than p2 it means p2 does
    // ran out of elements
    if p1.len() > p2.len() {
        return (false, false);
    }
    (true, true)
}

fn part1(input: String) -> String {
    let packets: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();
    let mut ordered = 0;
    let mut valid = Vec::new();
    for (i, pair) in packets.chunks(2).enumerate() {
        let p1 = parse_packet(pair[0]);
        let p2 = parse_packet(pair[1]);
        if compare_packets(&p1, &p2).1 {
            ordered += i + 1;
            valid.push(i + 1);
        }
    }
    ordered.to_string()
}

fn part2(_input: String) -> String {
    "Not implemented".to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
