use std::fmt::Display;
use std::ops::Add;

fn main() {
    let p0 = Point2D::new(10, 20);
    let p1 = Point2D::new(55, 65);

    let p3 = &p0 + &p1;
    println!("summ {:?}", p3);
    let p4 = p0 + p1;
    println!("summ just {:?}", p4);

    dbg!(p4.to_string());
    dbg!(Point2D::to_string(&p4));
    dbg!(ToString::to_string(&p4));

    dbg!(Point2D::default());
    dbg!(<Point2D as Default>::default());
}

#[derive(Copy, Clone, Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Default for Point2D {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "from display trait: ({}, {})", self.x, self.y)
    }
}

impl Point2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn to_string(&self) -> String {
        format!("from impl Point: ({}, {})", self.x, self.y)
    }

    fn default() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add for &Point2D {
    // для ссылок
    type Output = <Point2D as Add>::Output; // ??? как это читается?

    fn add(self, rhs: Self) -> Self::Output {
        Point2D::add(*self, *rhs) //  откуда у поинта взялся ассоциироанный метод
    }
}
