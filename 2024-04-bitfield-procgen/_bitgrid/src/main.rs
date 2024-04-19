use std::fmt;
use rand::Rng;

struct Calculator {
    expr: Box<Expr>,
}

impl Calculator {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let depth: u32 = rng.gen_range(20..=80);
        let expr = Self::generate_random_expr(&mut rng, depth);
        Calculator { expr }
    }

    fn calculate(&self, x: i32, y: i32) -> i32 {
        self.expr.evaluate(x, y)
    }


    fn generate_random_expr(rng: &mut impl Rng, depth: u32) -> Box<Expr> {
        let expr = if depth == 0 {
            match rng.gen_range(0..3) {
                0 => Expr::X,
                1 => Expr::Y,
                _ => Expr::Num(rng.gen_range(-10..=10)),
            }
        } else {
            match rng.gen_range(0..7) {
                0 => Expr::X,
                1 => Expr::Y,
                2 => Expr::Num(rng.gen_range(-10..=10)),
                3..=4 => {
                    let op = match rng.gen_range(0..3) {
                        0 => UnaryOp::Abs,
                        1 => UnaryOp::Negate,
                        2 => UnaryOp::BitwiseNot,
                        _ => unreachable!(),
                    };
                    let operand = Self::generate_random_expr(rng, depth - 1);
                    Expr::Unary(op, operand)
                },
                5..=6 => {
                    let op = match rng.gen_range(0..8) {
                        0 => BinaryOp::Add,
                        1 => BinaryOp::Sub,
                        2 => BinaryOp::Mul,
                        3 => BinaryOp::Div,
                        4 => BinaryOp::Mod,
                        5 => BinaryOp::And,
                        6 => BinaryOp::Or,
                        7 => BinaryOp::Xor,
                        _ => unreachable!()
                    };
                    let left = Self::generate_random_expr(rng, depth - 1);
                    let right = Self::generate_random_expr(rng, depth - 1);
                    Expr::Binary(op, left, right)
                },
                _ => unreachable!(),
            }
        };

        Box::new(expr)
    }
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::And => "&",
            BinaryOp::Or  => "|",
            BinaryOp::Xor => "^",
        };

        write!(f, "{repr}")
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            UnaryOp::Abs        => "abs",
            UnaryOp::Negate     => "-",
            UnaryOp::BitwiseNot => "~",
        };

        write!(f, "{repr}")
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::X => write!(f, "x"),
            Expr::Y => write!(f, "y"),
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Binary(op, left, right) => write!(f, "({} {} {})", left, op, right),
            Expr::Unary(op, expr) => {
                if matches!(op, UnaryOp::Negate | UnaryOp::BitwiseNot) {
                    write!(f, "{}{}", op, expr)
                } else {
                    write!(f, "{}({})", op, expr)
                }
            },
        }
    }
}



enum UnaryOp {
    Abs,
    Negate,
    BitwiseNot,
}

enum Expr {
    X,
    Y,
    Num(i32),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
}

impl Expr {
    fn evaluate(&self, x: i32, y: i32) -> i32 {
        match self {
            Expr::X => x,
            Expr::Y => y,
            Expr::Num(n) => *n,
            Expr::Binary(op, left, right) => {
                let left_val = left.evaluate(x, y);
                let right_val = right.evaluate(x, y);
                match op {
                    BinaryOp::Add => left_val + right_val,
                    BinaryOp::Sub => left_val - right_val,
                    BinaryOp::Mul => left_val * right_val,
                    BinaryOp::Div => match (left_val, right_val) {
                        (0, 0) => 1,
                        (_, 0) => 0,
                        (_, _) => left_val / right_val,
                    },
                    BinaryOp::Mod => {
                        if right_val == 0 {
                            0
                        } else {
                            left_val % right_val
                        }
                    }
                    BinaryOp::And => left_val & right_val,
                    BinaryOp::Or => left_val | right_val,
                    BinaryOp::Xor => left_val ^ right_val,
                }
            }
            Expr::Unary(op, expr) => {
                let val = expr.evaluate(x, y);
                match op {
                    UnaryOp::Abs => val.abs(),
                    UnaryOp::Negate => -val,
                    UnaryOp::BitwiseNot => !val,
                }
            }
        }
    }
}

fn main() {
    let calc = Calculator::new();

    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut grid = vec![vec![0; 256]; 256];

    for y in 0..256 {
        for x in 0..256 {
            let result = calc.calculate(x as i32, y as i32);
            if result < min {
                min = result;
            }
            if result > max {
                max = result;
            }
            grid[y][x] = result;
        }
    }

    for row in grid.iter_mut() {
        for val in row.iter_mut() {
            *val = ((*val - min) * 255) / (max - min);
        }
    }

    // println!("{grid:?}");
    println!("{}", calc.expr);
}
