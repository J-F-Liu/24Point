use num_rational::Rational32;
use num_traits::Zero;
use std::fmt;
mod expr;
use self::expr::*;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Calculation {
    result: Rational32,
    expression: Expr,
}

impl From<i32> for Calculation {
    fn from(n: i32) -> Calculation {
        Calculation {
            result: n.into(),
            expression: n.into(),
        }
    }
}

impl fmt::Display for Calculation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.expression, self.result)
    }
}

pub fn combine(numbers: Vec<Calculation>) -> Vec<Vec<Calculation>> {
    let mut selected: Vec<Vec<Calculation>> = vec![];
    let n = numbers.len();
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let mut combination = numbers.clone();
            combination.swap(0, i);
            combination.swap(1, j);
            if combination[0].result > combination[1].result {
                combination.swap(0, 1);
            }
            // select the combination and avoid duplication
            if selected.iter().all(|nums| {
                nums[0].result != combination[0].result || nums[1].result != combination[1].result
            }) {
                selected.push(combination);
            }
        }
    }
    selected
}

pub fn reduce(numbers: Vec<Calculation>) -> Vec<Vec<Calculation>> {
    let mut reduced = vec![
        concat(create_calculation(&numbers, '+', 0, 1), &numbers[2..]),
        concat(create_calculation(&numbers, '-', 1, 0), &numbers[2..]),
        concat(create_calculation(&numbers, '*', 0, 1), &numbers[2..]),
    ];
    if !numbers[0].result.is_zero() {
        reduced.push(concat(
            create_calculation(&numbers, '/', 1, 0),
            &numbers[2..],
        ));
    }
    if !numbers[1].result.is_zero() && numbers[0].result != numbers[1].result {
        reduced.push(concat(
            create_calculation(&numbers, '/', 0, 1),
            &numbers[2..],
        ));
    }
    reduced
}

fn create_calculation(
    numbers: &Vec<Calculation>,
    operator: char,
    left: usize,
    right: usize,
) -> Calculation {
    Calculation {
        result: match operator {
            '+' => numbers[left].result + numbers[right].result,
            '-' => numbers[left].result - numbers[right].result,
            '*' => numbers[left].result * numbers[right].result,
            '/' => numbers[left].result / numbers[right].result,
            _ => panic!(),
        },
        expression: merge_expr(
            numbers[left].expression.clone(),
            operator,
            numbers[right].expression.clone(),
        ),
    }
}

fn concat<T: Clone>(item: T, slice: &[T]) -> Vec<T> {
    let mut items = vec![item];
    items.extend_from_slice(slice);
    items
}

fn calculate(numbers: Vec<Calculation>, solutions: &mut Vec<Calculation>, total: &mut usize) {
    for combination in combine(numbers) {
        for reduced in reduce(combination) {
            *total += 1;
            if reduced.len() > 1 {
                calculate(reduced, solutions, total);
            } else if reduced[0].result == 24.into() {
                if !solutions
                    .iter()
                    .any(|solution| solution.to_string() == reduced[0].to_string())
                {
                    solutions.push(reduced[0].clone());
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn solve(numbers: Vec<i32>) -> String {
    let mut solutions = vec![];
    let mut total = 0;
    let calculations = numbers.into_iter().map(|num| num.into()).collect();
    calculate(calculations, &mut solutions, &mut total);
    println!("{}", total);
    let formulas: Vec<String> = solutions.iter().map(|s| s.to_string()).collect();
    formulas.join(";")
}

#[test]
fn test_solve() {
    println!("{}", solve(vec![3, 3, 4, 4]));
}
