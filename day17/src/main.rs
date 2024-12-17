use regex::Regex;
use std::fs;
use std::sync::LazyLock;
use std::time::Instant;

static REGISTER_A_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Register A: (-?\d+)").unwrap());
static REGISTER_B_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Register B: (-?\d+)").unwrap());
static REGISTER_C_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Register C: (-?\d+)").unwrap());

static PROGRAM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Program: (.+)").unwrap());

fn main() {
    const PATH: &str = "day17/src/day17_input.txt";

    // let mut a: u64 = 0;
    // loop {
    //     if ((((a % 8) ^ 2) ^ (a / 2_u64.pow((a % 8) ^ 2))) ^ 7) % 8 == 4 {
    //         println!("{a}")
    //     }
    //     a += 1;
    // }

    //part1(PATH);
    part2(PATH);
}

fn part2(path: &str) {
    let input = fs::read_to_string(&path).unwrap();
    let (program, (_, register_b, register_c)) = parse_program(&input);

    let mut register_a = 0;
    let start = Instant::now();
    loop {
        if register_a % 100_000_000 == 0 {
            println!("Trying {}: {:?}", register_a, start.elapsed());
        }

        if let Some(output) = quine_search(&program, (register_a, register_b, register_c)) {
            if output == program {
                println!("{register_a}");
                break;
            }
        }
        register_a += 1;
    }
}

fn part1(path: &str) {
    let input = fs::read_to_string(&path).unwrap();
    let (program, registers) = parse_program(&input);
    let output = execute_program(&program, registers)
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>();
    println!("{}", output.join(","));
}

fn parse_program(input: &str) -> (Vec<u64>, (u64, u64, u64)) {
    let register_a: u64 = REGISTER_A_RE
        .captures(&input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let register_b: u64 = REGISTER_B_RE
        .captures(&input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let register_c: u64 = REGISTER_C_RE
        .captures(&input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let program_captures = PROGRAM_RE.captures(&input).unwrap();
    let program = program_captures
        .get(1)
        .unwrap()
        .as_str()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (program, (register_a, register_b, register_c))
}

fn execute_program(program: &[u64], registers: (u64, u64, u64)) -> Vec<u64> {
    let (mut register_a, mut register_b, mut register_c) = registers;
    let mut output = vec![];

    let mut i_pointer = 0;
    while i_pointer < program.len() {
        let opcode = program[i_pointer];
        let operand = program[i_pointer + 1];

        match opcode {
            0 => {
                let numerator = register_a;
                let denominator = 2_u64
                    .pow(
                        get_combo_operand_val(operand, (register_a, register_b, register_c)) as u32,
                    );
                register_a = numerator / denominator;
                i_pointer += 2;
            }
            1 => {
                let new = register_b ^ operand;
                register_b = new;
                i_pointer += 2;
            }
            2 => {
                let new = get_combo_operand_val(operand, (register_a, register_b, register_c)) % 8;
                register_b = new;
                i_pointer += 2;
            }
            3 => {
                if register_a != 0 {
                    i_pointer = operand as usize;
                } else {
                    i_pointer += 2;
                }
            }
            4 => {
                let new = register_b ^ register_c;
                register_b = new;
                i_pointer += 2;
            }
            5 => {
                let new_output =
                    get_combo_operand_val(operand, (register_a, register_b, register_c)) % 8;
                output.push(new_output);
                i_pointer += 2;
            }
            6 => {
                let numerator = register_a;
                let denominator = 2_u64
                    .pow(
                        get_combo_operand_val(operand, (register_a, register_b, register_c)) as u32,
                    ) as u64;
                register_b = numerator / denominator;
                i_pointer += 2;
            }
            7 => {
                let numerator = register_a;
                let denominator = 2_u64
                    .pow(
                        get_combo_operand_val(operand, (register_a, register_b, register_c)) as u32,
                    );
                register_c = numerator / denominator;
                i_pointer += 2;
            }
            _ => panic!("Unknown opcode {opcode}"),
        }
    }

    output
}

fn quine_search(program: &[u64], registers: (u64, u64, u64)) -> Option<Vec<u64>> {
    let (mut register_a, mut register_b, mut register_c) = registers;
    let register_a_start = register_a;
    let mut output = vec![];
    let mut output_idx = 0usize;

    let mut i_pointer = 0;
    while i_pointer < program.len() {
        let opcode = program[i_pointer];
        let operand = program[i_pointer + 1];

        match opcode {
            0 => {
                let numerator = register_a;
                let denominator = 2_u64
                    .pow(
                        get_combo_operand_val(operand, (register_a, register_b, register_c)) as u32,
                    );
                register_a = numerator / denominator;
                i_pointer += 2;
            }
            1 => {
                let new = register_b ^ operand;
                register_b = new;
                i_pointer += 2;
            }
            2 => {
                let new = get_combo_operand_val(operand, (register_a, register_b, register_c)) % 8;
                register_b = new;
                i_pointer += 2;
            }
            3 => {
                if register_a != 0 {
                    i_pointer = operand as usize;
                } else {
                    i_pointer += 2;
                }
            }
            4 => {
                let new = register_b ^ register_c;
                register_b = new;
                i_pointer += 2;
            }
            5 => {
                let new_output =
                    get_combo_operand_val(operand, (register_a, register_b, register_c)) % 8;
                output.push(new_output);

                if program[output_idx] != new_output {
                    return None;
                }
                if output_idx > 8 {
                    println!("{register_a_start} {:?} {:?}", program, output);
                }
                output_idx += 1;
                i_pointer += 2;
            }
            6 => {
                let numerator = register_a;
                let denominator = 2_u64
                    .pow(
                        get_combo_operand_val(operand, (register_a, register_b, register_c)) as u32,
                    );
                register_b = numerator / denominator;
                i_pointer += 2;
            }
            7 => {
                let numerator = register_a;
                let denominator = 2_u64
                    .pow(
                        get_combo_operand_val(operand, (register_a, register_b, register_c)) as u32,
                    );
                register_c = numerator / denominator;
                i_pointer += 2;
            }
            _ => panic!("Unknown opcode {opcode}"),
        }
    }

    Some(output)
}

#[inline(always)]
fn get_combo_operand_val(operand: u64, registers: (u64, u64, u64)) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        _ => panic!("Unknown combo operand {operand}"),
    }
}
