use num_rational::Rational32;
use num_traits::Zero;
use std::env;
use std::fmt;

#[derive(Clone)]
struct Calculation {
  result: Rational32,
  expression: String,
}

impl From<i32> for Calculation {
  fn from(n: i32) -> Calculation {
    Calculation {
      result: n.into(),
      expression: n.to_string(),
    }
  }
}

impl fmt::Display for Calculation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} = {}", self.expression, self.result)
  }
}

fn combine(numbers: Vec<Calculation>) -> Vec<Vec<Calculation>> {
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
      if selected.iter().all(|nums| {
        nums[0].result != combination[0].result || nums[1].result != combination[1].result
      }) {
        selected.push(combination);
      }
    }
  }
  selected
}

fn reduce(numbers: Vec<Calculation>) -> Vec<(Vec<Calculation>)> {
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
    expression: if numbers.len() > 2 && operator != '*' {
      format!(
        "({}{}{})",
        numbers[left].expression, operator, numbers[right].expression
      )
    } else {
      format!(
        "{}{}{}",
        numbers[left].expression, operator, numbers[right].expression
      )
    },
  }
}

fn concat<T: Clone>(item: T, slice: &[T]) -> Vec<T> {
  let mut items = vec![item];
  items.extend_from_slice(slice);
  items
}

fn calculate(numbers: Vec<Calculation>, solutions: &mut Vec<Calculation>) {
  for combination in combine(numbers) {
    for reduced in reduce(combination) {
      if reduced.len() > 1 {
        calculate(reduced, solutions);
      } else if reduced[0].result == 24.into() {
        if !solutions
          .iter()
          .any(|solution| solution.expression == reduced[0].expression)
        {
          solutions.push(reduced[0].clone());
        }
      }
    }
  }
}

fn solve(numbers: Vec<i32>) -> Vec<Calculation> {
  let mut solutions = vec![];
  let calculations = numbers.into_iter().map(|num| num.into()).collect();
  calculate(calculations, &mut solutions);
  solutions
}

fn main() {
  use std::borrow::Cow;
  let numbers: Vec<i32> = env::args()
    .skip(1)
    .map(|arg| i32::from_str_radix(&arg, 10).expect("Input valid numbers."))
    .collect();
  let solutions = solve(numbers);
  println!(
    "Found {}.",
    match solutions.len() {
      0 => Cow::from("no solution"),
      1 => Cow::from("1 solution"),
      n @ _ => Cow::from(format!("{} solutions", n)),
    }
  );
  for solution in solutions {
    println!("{}", solution);
  }
}
