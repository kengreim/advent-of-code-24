use regex::Regex;
use std::fs;
use std::sync::LazyLock;
use std::time::Instant;

static REGISTER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Register \w: (-?\d+)").unwrap());

static PROGRAM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Program: (.+)").unwrap());

fn main() {
    const PATH: &str = "day17/src/day17_input.txt";

    //part1(PATH);
    part2(PATH);
}

fn part2(path: &str) {
    let input = fs::read_to_string(&path).unwrap();

    let (program, (_, register_b, register_c)) = parse_program(&input);

    let start = Instant::now();
    let mut register_a = 0;

    for i in 0..=(u32::max as u64) {
        if i > 0 && i % 100_000_000 == 0 {
            println!("Trying {}: {:?}", i, start.elapsed());
        }

        register_a = (i << 16) | 15375;
        if let Some(output) = quine_search(&program, (register_a, register_b, register_c), Some(16))
        {
            break;
        }
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
    let registers = REGISTER_RE
        .captures_iter(&input)
        .map(|c| c.get(1).unwrap().as_str().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let program_captures = PROGRAM_RE.captures(&input).unwrap();
    let program = program_captures
        .get(1)
        .unwrap()
        .as_str()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (program, (registers[0], registers[1], registers[2]))
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
                let new = get_combo_operand_val(operand, (register_a, register_b, register_c)) & 7;
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
                    get_combo_operand_val(operand, (register_a, register_b, register_c)) & 7;
                output.push(new_output);
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

    output
}

fn quine_search(
    program: &[u64],
    registers: (u64, u64, u64),
    output_index_stop: Option<usize>,
) -> Option<Vec<u64>> {
    let (mut register_a, mut register_b, mut register_c) = registers;
    let register_a_start = register_a;
    let mut output = vec![];
    let mut output_idx = 0usize;

    let output_index_stop_int = output_index_stop.unwrap_or(usize::MAX);

    let mut i_pointer = 0;
    while i_pointer < program.len() {
        match program[i_pointer] {
            0 => {
                let operand = program[i_pointer + 1];
                let numerator = register_a;
                let denominator = 2_u64
                    .pow(
                        get_combo_operand_val(operand, (register_a, register_b, register_c)) as u32,
                    );
                register_a = numerator / denominator;
                i_pointer += 2;
            }
            1 => {
                let operand = program[i_pointer + 1];
                let new = register_b ^ operand;
                register_b = new;
                i_pointer += 2;
            }
            2 => {
                let operand = program[i_pointer + 1];
                let new = get_combo_operand_val(operand, (register_a, register_b, register_c)) & 7;
                register_b = new;
                i_pointer += 2;
            }
            3 => {
                let operand = program[i_pointer + 1];
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
                let operand = program[i_pointer + 1];
                let new_output =
                    get_combo_operand_val(operand, (register_a, register_b, register_c)) & 7;
                output.push(new_output);

                if program[output_idx] != new_output {
                    return None;
                }

                output_idx += 1;
                i_pointer += 2;

                if output_idx >= output_index_stop_int {
                    println!("Lowest Register A to get {output_index_stop_int} outputs correct is {register_a_start} {:?} {:?}", program, output);
                    return Some(output);
                }
            }
            6 => {
                let operand = program[i_pointer + 1];
                let numerator = register_a;
                let denominator = 2_u64
                    .pow(
                        get_combo_operand_val(operand, (register_a, register_b, register_c)) as u32,
                    );
                register_b = numerator / denominator;
                i_pointer += 2;
            }
            7 => {
                let operand = program[i_pointer + 1];
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

    if output_idx < output_index_stop_int {
        None
    } else {
        Some(output)
    }
}

#[inline(always)]
fn get_combo_operand_val(operand: u64, (a, b, c): (u64, u64, u64)) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Unknown combo operand {operand}"),
    }
}
