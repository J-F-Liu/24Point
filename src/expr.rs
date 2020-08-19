use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub enum Expr {
    Num(i32),
    Oper { oper: char, terms: Vec<Expr> },
}

impl Expr {
    pub fn oper(oper: char, mut terms: Vec<Expr>) -> Expr {
        if is_commutative(oper) {
            // terms.sort();
        }
        Expr::Oper { oper, terms }
    }
}

impl From<i32> for Expr {
    fn from(n: i32) -> Expr {
        Expr::Num(n)
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Expr) -> Ordering {
        match self {
            Expr::Num(n) => match other {
                Expr::Num(m) => n.cmp(m),
                Expr::Oper { .. } => Ordering::Less,
            },
            Expr::Oper { .. } => match other {
                Expr::Num(_) => Ordering::Greater,
                Expr::Oper { .. } => self.to_string().cmp(&other.to_string()),
            },
        }
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Expr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Oper { oper, terms } => {
                let mut expr = String::new();
                let mut first = true;
                for term in terms {
                    if first {
                        first = false;
                    } else {
                        expr.push(*oper);
                    }
                    match term {
                        Expr::Num(n) => expr.push_str(&n.to_string()),
                        Expr::Oper {
                            oper: oper_sub,
                            terms: _,
                        } => {
                            if precedence(*oper_sub) > precedence(*oper)
                                || *oper == *oper_sub && is_commutative(*oper)
                            {
                                expr.push_str(&term.to_string());
                            } else {
                                expr.push_str(&format!("({})", term.to_string()));
                            }
                        }
                    }
                }
                write!(f, "{}", expr)
            }
        }
    }
}

pub fn merge_expr(left: Expr, oper: char, right: Expr) -> Expr {
    match left {
        Expr::Num(_) => match right {
            Expr::Num(_) => Expr::oper(oper, vec![left, right]),
            Expr::Oper {
                oper: op_right,
                mut terms,
            } => {
                if oper == op_right && is_commutative(oper) {
                    terms.insert(0, left);
                    Expr::oper(oper, terms)
                } else {
                    Expr::Oper {
                        oper,
                        terms: vec![
                            left,
                            Expr::Oper {
                                oper: op_right,
                                terms,
                            },
                        ],
                    }
                }
            }
        },
        Expr::Oper {
            oper: op_left,
            mut terms,
        } => match right {
            Expr::Num(_) => {
                if oper == op_left {
                    terms.push(right);
                    Expr::oper(oper, terms)
                } else {
                    Expr::oper(
                        oper,
                        vec![
                            Expr::Oper {
                                oper: op_left,
                                terms,
                            },
                            right,
                        ],
                    )
                }
            }
            Expr::Oper {
                oper: op_right,
                terms: mut terms_right,
            } => {
                if oper == op_left && oper == op_right && is_commutative(oper) {
                    terms.append(&mut terms_right);
                    Expr::oper(oper, terms)
                } else if oper == op_left && is_commutative(oper) {
                    terms.push(Expr::Oper {
                        oper: op_right,
                        terms: terms_right,
                    });
                    Expr::oper(oper, terms)
                } else if oper == op_right && is_commutative(oper) {
                    terms_right.insert(
                        0,
                        Expr::Oper {
                            oper: op_left,
                            terms,
                        },
                    );
                    Expr::oper(oper, terms_right)
                } else {
                    Expr::oper(
                        oper,
                        vec![
                            Expr::Oper {
                                oper: op_left,
                                terms,
                            },
                            Expr::Oper {
                                oper: op_right,
                                terms: terms_right,
                            },
                        ],
                    )
                }
            }
        },
    }
}

fn is_commutative(oper: char) -> bool {
    match oper {
        '+' | '*' => true,
        _ => false,
    }
}

fn precedence(oper: char) -> i32 {
    match oper {
        '+' | '-' => 1,
        _ => 2,
    }
}

#[test]
fn test_expr() {
    let add_expr = Expr::oper('+', vec![3.into(), 5.into()]);
    let mul_expr1 = Expr::oper('*', vec![4.into(), 8.into(), 6.into()]);
    let mul_expr2 = Expr::oper('*', vec![40.into(), 60.into(), 80.into()]);
    let mul_expr3 = merge_expr(mul_expr1.clone(), '*', mul_expr2);
    let mul_expr4 = merge_expr(mul_expr1.clone(), '+', add_expr.clone());
    let mul_expr5 = merge_expr(mul_expr1.clone(), '*', add_expr.clone());
    assert_eq!(add_expr.to_string(), "3+5");
    assert_eq!(mul_expr1.to_string(), "4*6*8");
    assert_eq!(mul_expr3.to_string(), "4*6*8*40*60*80");
    assert_eq!(mul_expr4.to_string(), "3+5+4*6*8");
    assert_eq!(mul_expr5.to_string(), "4*6*8*(3+5)");
}
