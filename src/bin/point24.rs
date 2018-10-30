use point24::solve;
use std::env;

fn main() {
  use std::borrow::Cow;
  let numbers: Vec<i32> = env::args()
    .skip(1)
    .map(|arg| {
      i32::from_str_radix(&arg, 10).unwrap_or_else(|_| {
        println!("{} is not a valid numbers.", arg);
        ::std::process::exit(1);
      })
    })
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
  for solution in solutions.split(";") {
    println!("{}", solution);
  }
}
