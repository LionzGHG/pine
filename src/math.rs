
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn in_area(&self, p1: Point, p2: Point) -> bool {
        let min_x = p1.x().min(p2.x());
        let max_x = p1.x().max(p2.x());
        let min_y = p1.y().min(p2.y());
        let max_y = p1.y().max(p2.y());

        self.x() >= min_x && self.x() <= max_x &&
        self.y() >= min_y && self.y() <= max_y
    }
}