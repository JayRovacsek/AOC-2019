#[derive(Clone)]
pub struct Interpreter {
    phase_setting_consumed: bool,
    phase_setting: Option<i64>,
    input_vec: Vec<i64>,
    head: usize,
    relative_base: i64,
}

impl Interpreter {
    pub fn new(phase_setting: Option<i64>, input_vec: Vec<i64>, head: usize) -> Interpreter {
        match phase_setting {
            Some(i) => Interpreter {
                phase_setting_consumed: false,
                phase_setting: Some(i),
                input_vec,
                head,
                relative_base: 0,
            },
            _ => Interpreter {
                phase_setting_consumed: true,
                phase_setting: None,
                input_vec,
                head,
                relative_base: 0,
            },
        }
    }

    pub fn run(&mut self, input_code: i64) -> Vec<i64> {
        use OpCode::*;
        let mut outputs: Vec<i64> = Vec::new();
        loop {
            let x = Operation::parse(self.input_vec.clone(), self.head);
            if x.opcode == End {
                break;
            };
            let result = if self.phase_setting_consumed {
                x.execute(
                    self.input_vec.clone(),
                    self.head,
                    input_code,
                    self.relative_base,
                )
            } else {
                self.phase_setting_consumed = true;
                x.execute(
                    self.input_vec.clone(),
                    self.head,
                    self.phase_setting.unwrap_or(0),
                    self.relative_base,
                )
            };
            self.input_vec = result.0;
            self.head = result.1;
            if result.2.is_some() {
                outputs.push(result.2.unwrap());
            };
            self.relative_base = result.3;
        }
        outputs
    }

    pub fn run_with_modified_registers(&mut self, input_code: i64, n: i64, v: i64) -> Vec<i64> {
        let mut outputs: Vec<i64> = Vec::new();
        self.input_vec[1] = n;
        self.input_vec[2] = v;
        let windows: Vec<usize> = (0..self.input_vec.len() / 4).collect();
        windows.iter().for_each(|x| {
            let z = Operation::parse(self.input_vec.clone(), x * 4);
            let result = if self.phase_setting_consumed {
                z.execute(
                    self.input_vec.clone(),
                    self.head,
                    input_code,
                    self.relative_base,
                )
            } else {
                self.phase_setting_consumed = true;
                z.execute(
                    self.input_vec.clone(),
                    self.head,
                    self.phase_setting.unwrap_or(0),
                    self.relative_base,
                )
            };
            self.input_vec = result.0;
            self.head = result.1;
            if result.2.is_some() {
                outputs.push(result.2.unwrap());
            }
        });
        outputs
    }

    pub fn run_one_output(&mut self, input_code: Option<i64>) -> Option<i64> {
        use OpCode::*;
        let mut output: Option<i64> = None;
        loop {
            let x = Operation::parse(self.input_vec.clone(), self.head);
            if x.opcode == End {
                break;
            };
            let result = if self.phase_setting_consumed {
                x.execute(
                    self.input_vec.clone(),
                    self.head,
                    input_code.unwrap(),
                    self.relative_base,
                )
            } else {
                self.phase_setting_consumed = true;
                x.execute(
                    self.input_vec.clone(),
                    self.head,
                    self.phase_setting.unwrap_or(0),
                    self.relative_base,
                )
            };
            self.input_vec = result.0;
            self.head = result.1;
            self.relative_base = result.3;
            if result.2.is_some() {
                output = Some(result.2.unwrap());
                break;
            }
        }
        output
    }
}

#[derive(Debug)]
struct Operation {
    opcode: OpCode,
    parameters: Vec<ParameterMode>,
}

impl Operation {
    fn parse(input_vec: Vec<i64>, head: usize) -> Operation {
        use OpCode::*;
        let input = input_vec[head];
        let o = match input % 100 {
            n if n < 9 => match n {
                1 => Addition,
                2 => Multiplication,
                3 => Input,
                4 => Output,
                5 => JumpIfTrue,
                6 => JumpIfFalse,
                7 => LessThan,
                8 => Equals,
                _ => End,
            },
            n if n >= 9 => match n {
                9 => RelativeAdjust,
                _ => End,
            },
            _ => End,
        };
        Operation {
            opcode: o.clone(),
            parameters: match o {
                End => vec![],
                _ => Operation::parse_parameters(input_vec[head], o),
            },
        }
    }

    fn parse_parameters(input: i64, opcode: OpCode) -> Vec<ParameterMode> {
        use OpCode::*;
        use ParameterMode::*;
        match opcode {
            Input | Output | RelativeAdjust => match input / 100 {
                0 => vec![Position],
                1 => vec![Immediate],
                2 => vec![Relative],
                _ => panic!(
                    "Attempted to parse an instruction that we otherwise don't yet know about: {}",
                    input
                ),
            },
            JumpIfTrue | JumpIfFalse => {
                let mut instruction: Vec<_> = (input / 100)
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect();
                while instruction.len() < 2 {
                    match instruction.len() {
                        n if n < 2 => instruction.insert(0, 0),
                        _ => break,
                    }
                }
                instruction
                .iter()
                .rev()
                .map(|x| match x {
                    0 => Position,
                    1 => Immediate,
                    2 => Relative,
                    _ => panic!("Attempted to parse an instruction that we otherwise don't yet know about: {}", x),
                })
                .collect()
            }
            _ => {
                let mut instruction: Vec<_> = (input / 100)
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect();
                while instruction.len() < 3 {
                    match instruction.len() {
                        n if n < 3 => instruction.insert(0, 0),
                        _ => break,
                    }
                }
                instruction
                    .iter()
                    .rev()
                    .map(|x| match x {
                        0 => Position,
                        1 => Immediate,
                        2 => Relative,
                        _ => panic!("Attempted to parse an instruction that we otherwise don't yet know about: {}", x),
                    })
                    .collect()
            }
        }
    }

    fn execute(
        &self,
        mut input_vec: Vec<i64>,
        mut head: usize,
        input_code: i64,
        mut relative_base: i64,
    ) -> (Vec<i64>, usize, Option<i64>, i64) {
        use OpCode::*;
        use ParameterMode::*;
        let mut output: Option<i64> = None;
        let params: Vec<i64> = self
            .parameters
            .iter()
            .enumerate()
            .map(|x| match x.1 {
                Immediate => (head + 1 + x.0) as i64,
                Position => {
                    if head + 1 + x.0 > input_vec.len() {
                        input_vec.resize((head + 1 + x.0) as usize, 0);
                    }
                    input_vec[head + 1 + x.0]
                },
                Relative => {
                    if head + 1 + relative_base as usize > input_vec.len() {
                        input_vec.resize((head + 2 + relative_base as usize) as usize, 0);
                    }
                    (input_vec[head + 1 + relative_base as usize]) as i64
                }
            })
            .collect();

        let expand_memory = params
            .iter()
            .map(|x| *x as usize > input_vec.len())
            .filter(|x| *x)
            .any(|x| x);

        // println!(
        //     "Expand memory: {:?}\nParams: {:?}\nLength: {:?}",
        //     expand_memory,
        //     params,
        //     input_vec.len()
        // );

        input_vec = if expand_memory {
            input_vec.resize((*params.iter().max().unwrap_or(&0_i64) + 1) as usize, 0);
            input_vec
        } else {
            input_vec
        };

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
                output = Some(input_vec[params[0] as usize]); // CHANGE BACK IF THIS IS WRONG
                                                              // output = Some(params[0]);
                head += 2;
            }
            RelativeAdjust => {
                relative_base += params[0];
                head += 2
            }
            End => output = None,
        }
        (input_vec, head, output, relative_base)
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
    RelativeAdjust,
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}
