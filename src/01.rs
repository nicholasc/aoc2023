use atoi::atoi;

#[rustfmt::skip]
const DIGITS: [[&str; 9]; 2] = [
    [ "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"],
];

fn main() {
    let data = include_str!("data/01");

    let part1 = parse(data, &|l| l.to_string());
    let part2 = parse(data, &literals);

    dbg!(part1, part2);
}

fn literals(input: &str) -> String {
    let mut output = input.to_string();
    let mut indices: Vec<usize> = vec![];
    let mut patches: Vec<(usize, usize)> = vec![];

    for (i, digit) in DIGITS[1].iter().enumerate() {
        for (pos, _) in output.clone().match_indices(digit) {
            let mut count = 0;
            #[rustfmt::skip]
            indices.iter().for_each(|v| if *v <= pos { count += 1 });

            let offset = count + pos;
            indices.push(offset);

            patches.push((offset, i));
        }
    }

    patches
        .into_iter()
        .for_each(|(a, b)| output.insert_str(a, DIGITS[0][b]));

    output
}

fn parse(data: &str, modifier: &dyn Fn(&str) -> String) -> usize {
    data.split("\r\n")
        .map(modifier)
        .map(|line| {
            let nums: Vec<u8> = line
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c as u8)
                .collect();
            atoi::<usize>(&[*nums.first().unwrap(), *nums.last().unwrap()]).unwrap()
        })
        .reduce(|a, e| a + e)
        .unwrap()
}
