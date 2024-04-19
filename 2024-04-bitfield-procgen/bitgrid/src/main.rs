use std::fmt;
use rand::Rng;

// use image::{RgbImage, Rgb};
use image::{DynamicImage, GrayImage, RgbImage, Rgb};

struct ProceduralGrid(Box<Expr>);

impl fmt::Display for ProceduralGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ProceduralGrid {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let depth = rng.gen_range(3..=8);
        let expr = Expr::generate(&mut rng, depth);

        ProceduralGrid(expr)
    }

    fn calc(&self, x: i32, y: i32) -> i32 {
        self.0.eval(x, y)
    }
}

enum Expr {
    X,
    Y,
    Num(i32),
    Binary(BinaryOp, Box<Expr>, Box<Expr>)
}

impl Expr {
    fn generate(rng: &mut impl Rng, depth: u32) -> Box<Expr> {
        let expr = if depth == 0 {
            match rng.gen_range(0..3) {
                0 => Expr::X,
                1 => Expr::Y,
                2 => Expr::Num(rng.gen_range(-64..=64)),
                _ => unreachable!(),
            }
        } else {
            match rng.gen_range(0..10) {
                0 => Expr::X,
                1 => Expr::Y,
                2..=5 => Expr::Num(rng.gen_range(-64..=64)),
                // we want binary operators to be more common
                _ => {
                    let left = Expr::generate(rng,
                        depth - 1);
                    let right  = Expr::generate(rng,
                            depth - 1);
                    let op = match rng.gen_range(0..20) {
                        0 => BinaryOp::Add,
                        1 => BinaryOp::Sub,
                        2..=5 =>   BinaryOp::And,
                        6..=9 =>   BinaryOp::Or,
                        10..=14 => BinaryOp::Xor,
                        15..=19 => BinaryOp::Mod,
                        _ => unreachable!(),
                    };
                    Expr::Binary(op, left, right)
                }
            }
        };

        Box::new(expr)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::X => write!(f, "x"),
            Expr::Y => write!(f, "y"),
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Binary(op, left, right) => write!(f, "({left} {op} {right})"),
            // Expr::Unary(op, expr) => {
            //     if matches!(op, UnaryOp::Negate | UnaryOp::BitwiseNot) {
            //         write!(f, "{}{}", op, expr)
            //     } else {
            //         write!(f, "{}({})", op, expr)
            //     }
            // },
        }
    }
}

enum BinaryOp {
    Add,
    Sub,
    /// Modulo
    Mod,
    /// Bitwise AND
    And,
    /// Bitwise OR
    Or,
    /// Bitwise XOR
    Xor,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            // BinaryOp::Mul => "*",
            // BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::And => "&",
            BinaryOp::Or  => "|",
            BinaryOp::Xor => "^",
        };

        write!(f, "{repr}")
    }
}

impl Expr {
    fn eval(&self, x: i32, y: i32) -> i32 {
        match self {
            Expr::X => x,
            Expr::Y => y,
            Expr::Num(a) => *a,
            Expr::Binary(op, left, right) => {

                let left = left.eval(x, y);
                let right = right.eval(x, y);

                match op {
                    BinaryOp::Add => left + right,
                    BinaryOp::Sub => left - right,
                    BinaryOp::And => left & right,
                    BinaryOp::Or => left | right,
                    BinaryOp::Xor => left ^ right,
                    BinaryOp::Mod => left % right,
                }
            }
        }
    }
}

fn main() {
    let grid = ProceduralGrid::new();
    let mut buf = vec![vec![0; 256]; 256];

    let mut img = RgbImage::new(256, 256);

    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for y in 0..256 {
        for x in 0..256 {
            // let x = x as i64;
            // let y = y as i64;
            // let val = ((x ^ y) % 11) | ((x | y) % 17);
            // let val = (x | y) % 17;
            // let val: i64 = ((((x | 12) - (-x)) ^ (!(x % y))) & (((!y) - (20 +
            // x)) - (!(-y)))) % 9;

            let val = grid.calc(x, y);

            if val < min {
                min = val;
            }

            if val > max {
                max = val;
            }

            // if val > 0 {
            //     img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]))
            // }

            buf[x as usize][y as usize] = val;
        }
    }

    for row in buf.iter_mut() {
        for val in row.iter_mut() {
            *val = ((*val - min) * 255) / (max - min);
        }
    }

    for y in 0..256 {
        for x in 0..256 {
            let val = buf[x as usize][y as usize] as u8;

            img.put_pixel(x as u32, y as u32, Rgb([val, val, val]))

        }
    }

    let img = DynamicImage::ImageRgb8(img);
    // let img: GrayImage = .into_luma8();
    // img.resize(256, 256, image::imageops::FilterType::Nearest);

    img.save("grid.png").unwrap();

    println!("{grid}");
}
