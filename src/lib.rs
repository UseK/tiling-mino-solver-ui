use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Shape(Vec<Vec<bool>>);

impl Shape {
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
    fn is_wall(&self, x: usize, y: usize) -> bool {
        self.0[y][x]
    }
    pub fn count_wall(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.is_wall(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn count_vacant(&self) -> usize {
        self.width() * self.height() - self.count_wall()
    }
    pub fn put_on(&mut self, x: usize, y: usize, b: bool) {
        self.0[y][x] |= b;
    }
    pub fn coordinates(&self) -> Vec<(usize, usize, bool)> {
        let mut vs = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                vs.push((x, y, self.is_wall(x, y)))
            }
        }
        vs
    }
}

impl FromStr for Shape {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().map(|c| c != '.').collect::<Vec<bool>>())
                .collect(),
        ))
    }
}
