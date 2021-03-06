mod test;

use crate::intcode::interpreter::Interpreter;
use rayon::prelude::*;

pub fn solve() {
    let answer_a = generate_combinations(0, 4)
        .par_iter()
        .map(|x| {
            x.iter().fold(0, |y, z| {
                let mut interpreter = Interpreter::new(Some(*z as i64), INPUT_VEC.to_vec(), 0);
                interpreter.run_one_output(Some(y)).unwrap_or(0)
            })
        })
        .max();
    println!("The answer for day 7, part a is: {:?}", answer_a);
    let answer_b = generate_combinations(5, 9)
        .par_iter()
        .map(|x| {
            let mut input: Option<i64> = Some(0);
            let mut interpreter_a = Interpreter::new(Some(x[0] as i64), INPUT_VEC.to_vec(), 0);
            let mut interpreter_b = Interpreter::new(Some(x[1] as i64), INPUT_VEC.to_vec(), 0);
            let mut interpreter_c = Interpreter::new(Some(x[2] as i64), INPUT_VEC.to_vec(), 0);
            let mut interpreter_d = Interpreter::new(Some(x[3] as i64), INPUT_VEC.to_vec(), 0);
            let mut interpreter_e = Interpreter::new(Some(x[4] as i64), INPUT_VEC.to_vec(), 0);
            let mut done = false;
            loop {
                let new_input = interpreter_a.run_one_output(input);
                if new_input.is_some() {
                    input = new_input
                } else {
                    done = true
                }

                let new_input = interpreter_b.run_one_output(input);
                if new_input.is_some() {
                    input = new_input
                } else {
                    done = true
                }

                let new_input = interpreter_c.run_one_output(input);
                if new_input.is_some() {
                    input = new_input
                } else {
                    done = true
                }

                let new_input = interpreter_d.run_one_output(input);
                if new_input.is_some() {
                    input = new_input
                } else {
                    done = true
                }

                let new_input = interpreter_e.run_one_output(input);
                if new_input.is_some() {
                    input = new_input
                } else {
                    done = true
                }

                if done {
                    break;
                }
            }
            input.unwrap_or(0_i64)
        })
        .max();
    println!("The answer for day 7, part b is: {:?}", answer_b);
}

// This is horrible and I wish to rewrite it when I can think about it more.
fn generate_combinations(lower: i32, upper: i32) -> Vec<Vec<i32>> {
    let mut combinations: Vec<Vec<i32>> = Vec::new();
    for a in lower..=upper {
        for b in lower..=upper {
            for c in lower..=upper {
                for d in lower..=upper {
                    for e in lower..=upper {
                        if a != b && b != c && c != d && d != e {
                            let v = vec![a, b, c, d, e];
                            let mut v2 = v.clone();
                            v2.sort();
                            v2.dedup();
                            if v.len() == v2.len() {
                                combinations.push(vec![a, b, c, d, e])
                            }
                        }
                    }
                }
            }
        }
    }
    combinations
}

const INPUT_VEC: [i64; 511] = [
    3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 42, 59, 76, 85, 106, 187, 268, 349, 430, 99999, 3, 9,
    102, 3, 9, 9, 1001, 9, 2, 9, 1002, 9, 3, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 102, 3, 9, 9, 101,
    3, 9, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 102, 3, 9, 9, 1001, 9, 4, 9, 1002, 9, 5, 9, 4, 9, 99,
    3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 1002, 9, 2, 9, 1001, 9, 4, 9, 1002, 9, 2, 9,
    4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
    1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9,
    4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9,
    1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9,
    4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101,
    2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4,
    9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2,
    9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3,
    9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9,
    2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
    9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9,
    4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
    102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9,
    4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
    1002, 9, 2, 9, 4, 9, 99,
];
