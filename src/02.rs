use std::collections::HashMap;

fn main() {
    let mut ids = vec![];
    let mut part2 = 0;

    let data = include_str!("data/02");
    data.split("\r\n").for_each(|l| {
        let mut map = HashMap::<&str, u32>::new();
        let split: Vec<&str> = l.split(":").collect();
        let id = split[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();

        let mut valid = true;
        split[1].split(";").for_each(|s| {
            s.trim().split(",").for_each(|i| {
                let item: Vec<&str> = i.trim().split(" ").collect();
                let num = item[0].parse::<u32>().unwrap();

                match (item[1], num) {
                    ("red", num) if num > 12 => valid = false,
                    ("green", num) if num > 13 => valid = false,
                    ("blue", num) if num > 14 => valid = false,
                    _ => {}
                }

                map.entry(item[1])
                    .and_modify(|n| {
                        if *n < num {
                            *n = num
                        }
                    })
                    .or_insert(num);
            });
        });

        part2 += map.get("red").unwrap() * map.get("green").unwrap() * map.get("blue").unwrap();

        if valid {
            ids.push(id);
        }
    });

    let part1 = ids.into_iter().reduce(|a, e| a + e).unwrap();

    dbg!(part1, part2);
}