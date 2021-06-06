use std::fs;


fn main() {
    let content = fs::read_to_string("inputs/day10.txt").expect("Expected a file");
    let mut jolts: Vec<u32> = content
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    jolts.sort();
    // add first and last elements manually
    jolts.insert(0, 0);
    jolts.push(jolts.last().unwrap() + 3);

    let mut differences: Vec<u32> = vec![0, 0, 0, 0];
    for i in 1..jolts.len() {
        differences[(jolts[i] - jolts[i - 1]) as usize] += 1;
    }
    println!("Part 1: {}", differences[1] * differences[3]);

    let mut ways: Vec<u64> = vec![0; jolts.len()];
    // 1 way to get to the charging outlet
    ways[0] = 1;

    for current_adapter in 0..jolts.len() {
        let mut next_adapter = current_adapter + 1;
        while next_adapter < jolts.len() && jolts[next_adapter] - jolts[current_adapter] <= 3 {
            // if it's possible to get to next_adapter from current_adapter
            // then the number of ways to get to next_adapter is increased
            // by the number of ways to get to current_adapter.
            ways[next_adapter] += ways[current_adapter];
            next_adapter += 1;
        }
    }

    println!("Part 2: {}", ways.last().unwrap());
}

