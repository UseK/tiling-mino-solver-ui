use std::{collections::HashSet, fs::File, io::Read, path::Path, str::FromStr};

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
    pub fn toggle(&mut self, x: usize, y: usize) {
        self.0[y][x] = !self.0[y][x];
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

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Mino {
    name: char,
    pub shape: Shape,
}

impl Mino {
    pub fn new(name: char, shape: Shape) -> Self {
        Self { name, shape }
    }
    pub fn pretty_print(&self) {
        println!("------------");
        self.shape.0.iter().for_each(|bools| {
            let line = bools
                .iter()
                .map(|&b| if b { self.name } else { '.' })
                .collect::<String>();
            println!("{}", line)
        });
        println!("------------");
    }
    pub fn height(&self) -> usize {
        self.shape.height()
    }
    pub fn width(&self) -> usize {
        self.shape.width()
    }
    pub fn rotated(&self, rotation: &Rotation) -> Self {
        let new_raw_shape = match rotation {
            Rotation::Neutral => self.shape.clone(),
            Rotation::Left => {
                let mut right_shape = vec![vec![false; self.height()]; self.width()];
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        right_shape[self.width() - x - 1][y] = self.shape.is_wall(x, y);
                    }
                }
                Shape(right_shape)
            }
            Rotation::Right => {
                let mut left_shape = vec![vec![false; self.height()]; self.width()];
                for y in 0..self.height() {
                    (0..self.width()).for_each(|x| {
                        left_shape[x][self.height() - y - 1] = self.shape.is_wall(x, y);
                    });
                }
                Shape(left_shape)
            }
            Rotation::OneEighty => {
                let mut one_eighty_shape = vec![vec![false; self.width()]; self.height()];
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        one_eighty_shape[self.height() - y - 1][self.width() - x - 1] =
                            self.shape.is_wall(x, y);
                    }
                }
                Shape(one_eighty_shape)
            }
        };
        Self {
            shape: new_raw_shape,
            name: self.name,
        }
    }
    pub fn minos_from_path<P>(path: P) -> Vec<Self>
    where
        P: AsRef<Path>,
    {
        if path.as_ref().is_file() {
            Self::minos_from_text_path(path)
        } else if path.as_ref().is_dir() {
            Self::minos_from_directory_path(path)
        } else {
            panic!("Invalid path {:?}", path.as_ref());
        }
    }

    pub fn minos_from_directory_path<P>(directory_path: P) -> Vec<Self>
    where
        P: AsRef<Path>,
    {
        directory_path
            .as_ref()
            .read_dir()
            .unwrap()
            .flat_map(|entry| Self::minos_from_text_path(entry.unwrap().path()))
            .collect()
    }
    pub fn minos_from_text_path<P>(p: P) -> Vec<Self>
    where
        P: AsRef<Path>,
    {
        let mut buf = "".to_string();
        File::open(p).unwrap().read_to_string(&mut buf).unwrap();
        let lines: Vec<String> = buf.lines().map(|s| s.to_string()).collect();
        lines
            .split(|line| line.contains('-'))
            .flat_map(|block| {
                let count = block[0].parse::<usize>().unwrap();
                let s: String = block[1..].join("\n");
                vec![Mino::from_str(&s).unwrap(); count]
            })
            .collect()
    }
}

impl FromStr for Mino {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs: HashSet<char> = s.trim().chars().collect();
        cs.remove(&'.');
        cs.remove(&'\n');
        if cs.len() != 1 {
            println!("{}", s);
        }
        assert_eq!(cs.len(), 1);
        let name = cs.into_iter().collect::<Vec<char>>()[0];
        Ok(Self {
            name,
            shape: Shape::from_str(s)?,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Rotation {
    Neutral,
    Left,
    Right,
    OneEighty,
}
