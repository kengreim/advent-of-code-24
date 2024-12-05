use std::collections::HashMap;
use std::fs;

fn main() {
    const PATH: &str = "day5/src/day5_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();

    let rules = input
        .lines()
        .filter(|l| l.contains("|"))
        .map(|l| {
            let nums = l.split("|").collect::<Vec<&str>>();
            (nums[0], nums[1])
        })
        .collect::<Vec<_>>();

    // let rules_map = input
    //     .lines()
    //     .filter(|l| l.contains("|"))
    //     .map(|l| {
    //         let nums = l.split("|").collect::<Vec<&str>>();
    //         (nums[0], (nums[0], nums[1]))
    //     })
    //     .collect::<HashMap<_, _>>();

    let sequences = input
        .lines()
        .filter(|l| l.contains(","))
        .collect::<Vec<_>>();

    let mut safe = vec![];
    let mut unsafe_order = vec![];
    'outer: for sequence in sequences {
        // let applicable_rules = sequence
        //     .split(",")
        //     .map(|n| rules_map.get(n))
        //     .flatten()
        //     .collect::<Vec<_>>();

        //println!("Checking sequence: {:?}", sequence);
        //println!("Applying rules: {:?}", applicable_rules);

        for rule in rules.iter() {
            if !is_safe(rule, sequence) {
                unsafe_order.push(sequence);
                continue 'outer;
            }
        }
        safe.push(sequence);
    }

    let sum = safe
        .iter()
        .map(|s| {
            let nums = s.split(',').collect::<Vec<_>>();
            nums.get(nums.len() / 2).unwrap().parse::<i32>().unwrap()
        })
        .sum::<i32>();

    println!("{sum}");

    // PART 2

    //println!("{}", unsafe_order.len());
    let mut fixed = vec![];
    for sequence in unsafe_order.iter() {
        let mut sequence_vec = sequence.split(',').collect::<Vec<_>>();
        let mut safe = false;
        while !safe {
            for rule in rules.iter() {
                //println!("Before applying rule {} {}", rule.0, rule.1);
                match (
                    sequence_vec.iter().position(|r| *r == rule.0),
                    sequence_vec.iter().position(|r| *r == rule.1),
                ) {
                    (Some(a), Some(b)) => {
                        if a > b {
                            //println!("{:?}", sequence_vec);
                            let temp = sequence_vec[a];
                            sequence_vec.remove(a);
                            sequence_vec.insert(b, temp);

                            //println!("After");
                            //println!("{:?}", sequence_vec);
                        }
                    }
                    _ => (),
                }
            }

            safe = rules
                .iter()
                .map(|r| is_safe(r, &sequence_vec.join(",").as_str()))
                .all(|b| b);
            //println!("Safe check: {safe}");
        }
        fixed.push(sequence_vec);
    }

    //println!("{:?}", fixed);

    let sum2 = fixed
        .iter()
        .map(|nums| nums.get(nums.len() / 2).unwrap().parse::<i32>().unwrap())
        .sum::<i32>();
    println!("{sum2}");
}

fn is_safe(rule: &(&str, &str), sequence: &str) -> bool {
    match (sequence.find(rule.0), sequence.find(rule.1)) {
        (Some(a), Some(b)) => a < b,
        _ => true,
    }
}
