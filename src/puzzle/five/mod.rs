#[derive(Debug)]
struct Operation {
    opcode: OpCode,
    parameters: Vec<ParameterMode>,
}

impl Operation {
    fn parse(input_vec: Vec<i32>, head: usize) -> Operation {
        use OpCode::*;
        let input = input_vec[head];
        let opcode = match input % 10 {
            1 => Addition,
            2 => Multiplication,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            _ => End,
        };
        Operation {
            opcode: opcode.clone(),
            parameters: match opcode {
                End => vec![],
                _ => Operation::parse_parameters(input_vec[head], opcode),
            },
        }
    }

    fn parse_parameters(input: i32, opcode: OpCode) -> Vec<ParameterMode> {
        use OpCode::*;
        use ParameterMode::*;
        match opcode {
            Input | Output => {
                match ((input / 10) % 10) % 2 {
                    0 => vec![PositionMode],
                    _ => vec![ImmediateMode],
                }
            }
            JumpIfTrue | JumpIfFalse => {
                let mut instruction: Vec<_> = input
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap())
                .collect();
            instruction.pop();
            instruction.pop();
            while let x = instruction.len() {
                match x {
                    n if n < 2 => instruction.insert(0, 0),
                    _ => break,
                }
            }
            instruction
                .iter()
                .rev()
                .map(|x| match x {
                    0 => PositionMode,
                    1 => ImmediateMode,
                    _ => panic!("Attempted to parse an instruction that we otherwise don't yet know about: {}", x),
                })
                .collect()
            }
            _ => {
                let mut instruction: Vec<_> = input
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect();
                instruction.pop();
                instruction.pop();
                while let x = instruction.len() {
                    match x {
                        n if n < 3 => instruction.insert(0, 0),
                        _ => break,
                    }
                }
                instruction
                    .iter()
                    .rev()
                    .map(|x| match x {
                        0 => PositionMode,
                        1 => ImmediateMode,
                        _ => panic!("Attempted to parse an instruction that we otherwise don't yet know about: {}", x),
                    })
                    .collect()
            }
        }
    }

    fn execute(
        &self,
        mut input_vec: Vec<i32>,
        mut head: usize,
        input_code: i32,
    ) -> (Vec<i32>, usize, Option<i32>) {
        use OpCode::*;
        use ParameterMode::*;
        println!("Parsing {:?} as instructions", input_vec[head]);
        let mut output: Option<i32> = None;
        let params: Vec<i32> = self
            .parameters
            .iter()
            .enumerate()
            .map(|x| match x.1 {
                ImmediateMode => (head + 1 + x.0) as i32,
                PositionMode => input_vec[head + 1 + x.0],
            })
            .collect();

        println!(
            "Instructions suggest these registers are to be used {:?}",
            params
        );

        match self.opcode {
            Addition => {
                input_vec[params[2] as usize] =
                    input_vec[params[0] as usize] + input_vec[params[1] as usize];
                head += 4;
            }
            Multiplication => {
                input_vec[params[2] as usize] =
                    input_vec[params[0] as usize] * input_vec[params[1] as usize];
                head += 4;
            }
            Input => {
                println!("Set value {} as 1", input_vec[params[0] as usize]);
                input_vec[params[0] as usize] = input_code;
                head += 2;
            }
            JumpIfTrue => {
                if input_vec[params[0] as usize] != 0 {
                    head = input_vec[params[1] as usize] as usize;
                } else {
                    head += 3;
                }
            }
            JumpIfFalse => {
                if input_vec[params[0] as usize] == 0 {
                    head = input_vec[params[1] as usize] as usize;
                } else {
                    head += 3;
                }
            }
            LessThan => {
                input_vec[params[2] as usize] =
                    if input_vec[params[0] as usize] < input_vec[params[1] as usize] {
                        1
                    } else {
                        0
                    };
                head += 4
            }
            Equals => {
                input_vec[params[2] as usize] =
                    if input_vec[params[0] as usize] == input_vec[params[1] as usize] {
                        1
                    } else {
                        0
                    };
                head += 4
            }
            Output => {
                output = Some(input_vec[params[0] as usize]);
                println!("Output OpCode, Value: {}", input_vec[params[0] as usize]);
                head += 2;
            }
            End => panic!("Operation ending"),
        }
        (input_vec, head, output)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum OpCode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    End,
}

#[derive(Debug)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

pub fn solve() {
    let registers = INPUT_VEC.clone().to_vec();
    let answer_a = execute_instructions(registers.clone(), 1);
    println!("The answer for day 5, part a is: {:?}", answer_a);
    let answer_b = execute_instructions(registers, 5);
    println!("The answer for day 5, part b is: {:?}", answer_b);
}

fn execute_instructions(mut input_vec: Vec<i32>, input_code: i32) -> i32 {
    use OpCode::*;
    let mut head = 0;
    let mut outputs: Vec<i32> = Vec::new();
    loop {
        let op = Operation::parse(input_vec.clone(), head);
        if op.opcode == End {
            break;
        };
        let result = op.execute(input_vec, head, input_code);
        input_vec = result.0;
        head = result.1;
        if result.2.is_some() {
            outputs.push(result.2.unwrap());
        };
    }
    *outputs.last().unwrap_or(&0_i32)
}

#[test]
fn test_part_a() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(7_265_618, execute_instructions(INPUT_VEC.to_vec(), 1));
}

// Tests to ensure input is equal to 8: on true return 1,
// else return 0
#[test]
fn test_part_b_example_1() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        1,
        execute_instructions([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 8)
    );
    assert_eq!(
        0,
        execute_instructions([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 9)
    );
    assert_ne!(
        0,
        execute_instructions([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 8)
    );
    assert_ne!(
        1,
        execute_instructions([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 999)
    );
}

// Tests to ensure input is less than 8: on true return 1,
// else return 0
#[test]
fn test_part_b_example_2() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        1,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 7)
    );
    assert_eq!(
        1,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 0)
    );
    assert_eq!(
        0,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 9)
    );
    assert_eq!(
        0,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 10)
    );
    assert_ne!(
        0,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 6)
    );
    assert_ne!(
        0,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 2)
    );
    assert_ne!(
        1,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 99)
    );
}

// Tests to ensure input is equal to 8: on true return 1,
// else return 0
#[test]
fn test_part_b_example_3() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        1,
        execute_instructions([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 8)
    );
    assert_eq!(
        0,
        execute_instructions([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 9)
    );
    assert_ne!(
        0,
        execute_instructions([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 8)
    );
    assert_ne!(
        1,
        execute_instructions([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 999)
    );
}

// Tests to ensure input is less than 8: on true return 1,
// else return 0
#[test]
fn test_part_b_example_4() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        1,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 7)
    );
    assert_eq!(
        1,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 0)
    );
    assert_eq!(
        0,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 9)
    );
    assert_eq!(
        0,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 10)
    );
    assert_ne!(
        0,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 6)
    );
    assert_ne!(
        0,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 2)
    );
    assert_ne!(
        1,
        execute_instructions([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 99)
    );
}

const INPUT_VEC: [i32; 678] = [
    3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 59, 58, 224, 1001, 224, -3422, 224, 4,
    224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 224, 223, 223, 1101, 59, 30, 225, 1101, 53, 84,
    224, 101, -137, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 3, 224, 224, 1, 223, 224, 223, 1102,
    42, 83, 225, 2, 140, 88, 224, 1001, 224, -4891, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5,
    224, 1, 223, 224, 223, 1101, 61, 67, 225, 101, 46, 62, 224, 1001, 224, -129, 224, 4, 224, 1002,
    223, 8, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1102, 53, 40, 225, 1001, 35, 35, 224, 1001,
    224, -94, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223, 1101, 5, 73, 225,
    1002, 191, 52, 224, 1001, 224, -1872, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5, 224, 1,
    223, 224, 223, 102, 82, 195, 224, 101, -738, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 2,
    224, 1, 224, 223, 223, 1101, 83, 52, 225, 1101, 36, 77, 225, 1101, 9, 10, 225, 1, 113, 187,
    224, 1001, 224, -136, 224, 4, 224, 1002, 223, 8, 223, 101, 2, 224, 224, 1, 224, 223, 223, 4,
    223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105,
    1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105,
    1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225,
    225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225,
    225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1007, 226, 226, 224, 1002, 223, 2, 223,
    1006, 224, 329, 1001, 223, 1, 223, 1108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 344, 101,
    1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 359, 101, 1, 223, 223, 1108,
    677, 226, 224, 1002, 223, 2, 223, 1005, 224, 374, 1001, 223, 1, 223, 7, 677, 226, 224, 102, 2,
    223, 223, 1005, 224, 389, 1001, 223, 1, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1005, 224,
    404, 101, 1, 223, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 419, 101, 1, 223, 223,
    1008, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 434, 1001, 223, 1, 223, 1107, 677, 226, 224,
    1002, 223, 2, 223, 1005, 224, 449, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223,
    1005, 224, 464, 1001, 223, 1, 223, 8, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 479, 1001,
    223, 1, 223, 107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223, 1, 223, 7, 226,
    226, 224, 102, 2, 223, 223, 1005, 224, 509, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2, 223,
    223, 1005, 224, 524, 101, 1, 223, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 539,
    101, 1, 223, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 554, 101, 1, 223, 223, 1107,
    677, 677, 224, 1002, 223, 2, 223, 1005, 224, 569, 101, 1, 223, 223, 108, 226, 677, 224, 1002,
    223, 2, 223, 1006, 224, 584, 101, 1, 223, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224,
    599, 1001, 223, 1, 223, 8, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 614, 1001, 223, 1, 223,
    108, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 629, 1001, 223, 1, 223, 1007, 226, 677, 224,
    1002, 223, 2, 223, 1006, 224, 644, 101, 1, 223, 223, 1108, 226, 677, 224, 102, 2, 223, 223,
    1005, 224, 659, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 674, 1001,
    223, 1, 223, 4, 223, 99, 226,
];
