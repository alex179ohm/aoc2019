pub enum Path {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32),
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        let n = s[1..].parse::<i32>().expect("failed to parse path input");
        match s.chars().nth(0) {
            Some('R') => Path::Right(n),
            Some('U') => Path::Up(n),
            Some('D') => Path::Down(n),
            Some('L') => Path::Left(n),
            Some(_) => unreachable!("unespected path input"),
            None => unreachable!("unespected path input"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Point(i32, i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Debug, Clone)]
pub struct Wire {
    points: Vec<Point>,
}

impl Wire {
    pub fn new() -> Self {
        Wire {
            points: Vec::new(),
        }
    }

    fn right(&mut self) {
        if self.points.len() == 0 {
            self.points.push(Point(1, 0, 1));
        } else {
            let mut last = self.points[self.points.len() - 1].clone();
            last.0 += 1;
            last.2 += 1;
            self.points.push(last);
        }
    }

    fn up(&mut self) {
        if self.points.len() == 0 {
            self.points.push(Point(0, 1, 1));
        } else {
            let mut last = self.points[self.points.len() - 1].clone();
            last.1 += 1;
            last.2 += 1;
            self.points.push(last);
        }
    }

    fn down(&mut self) {
        if self.points.len() == 0 {
            self.points.push(Point(0, -1, 1));
        } else {
            let mut last = self.points[self.points.len() - 1].clone();
            last.1 -= 1;
            last.2 += 1;
            self.points.push(last);
        }
    }

    fn left(&mut self) {
        if self.points.len() == 0 {
            self.points.push(Point(-1, 0, 1));
        } else {
            let mut last = self.points[self.points.len() - 1].clone();
            last.0 -= 1;
            last.2 += 1;
            self.points.push(last);
        }
    }

    fn add_points(&mut self, p: Path) {
        match p {
            Path::Up(n) => {
                for _ in 0..n {
                    self.up()
                }
            }
            Path::Down(n) => {
                for _ in 0..n {
                    self.down()
                }
            }
            Path::Left(n) => {
                for _ in 0..n {
                    self.left()
                }
            }
            Path::Right(n) => {
                for _ in 0..n {
                    self.right()
                }
            }
        }
    }

    pub fn get_crosses(&self, other: Wire) -> Vec<(Point, Point)> {
        let mut res: Vec<(Point, Point)> = Vec::new();
        for point in &self.points {
            for other_point in &other.points {
                if point == other_point {
                    res.push((point.clone(), other_point.clone()));
                }
            }
        }
        res
    }
}

impl From<Vec<&str>> for Wire {
    fn from(v: Vec<&str>) -> Self {
        let mut wire =  Wire::new();
        for p in v { wire.add_points(Path::from(p)) }
        wire
    }
}

pub fn one(input: String) -> i32 {
    let mut wires: Vec<Wire> = Vec::new();
    for line in input.lines() {
        wires.push(Wire::from(line.split(',').into_iter().collect::<Vec<&str>>()));
    }
    let crosses = wires[0].get_crosses(wires[1].clone());
    let mut mds = crosses.iter().map(|x| (x.0).0.abs() + (x.0).1.abs()).collect::<Vec<i32>>();
    mds.sort();
    mds[0]
}

pub fn two(input: String) -> i32 {
    let mut wires: Vec<Wire> = Vec::new();
    for line in input.lines() {
        wires.push(Wire::from(line.split(',').into_iter().collect::<Vec<&str>>()));
    }
    let crosses = wires[0].get_crosses(wires[1].clone());
    let mut res = crosses.iter().map(|x| (x.0).2 + (x.1).2).collect::<Vec<i32>>();
    res.sort();
    res[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_manhattan_distance() {
        let wire = Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72".split(',').into_iter().collect::<Vec<&str>>());
        let wire2 = Wire::from("U62,R66,U55,R34,D71,R55,D58,R83".split(',').into_iter().collect::<Vec<&str>>());
        let crosses = wire.get_crosses(wire2);
        let mut mds = crosses.iter().map(|x| (x.0).0.abs() + (x.0).1.abs()).collect::<Vec<i32>>();
        mds.sort();
        assert_eq!(mds[0], 159);
    }

    #[test]
    fn test2_manhattan_distance() {
        let wire = Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".split(',').into_iter().collect::<Vec<&str>>());
        let wire2 = Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(',').into_iter().collect::<Vec<&str>>());
        let crosses = wire.get_crosses(wire2);
        let mut mds = crosses.iter().map(|x| (x.0).0.abs() + (x.0).1.abs()).collect::<Vec<i32>>();
        mds.sort();
        assert_eq!(mds[0], 135);
    }

    #[test]
    fn best_steps_test1() {
        let wire = Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72".split(',').into_iter().collect::<Vec<&str>>());
        let wire2 = Wire::from("U62,R66,U55,R34,D71,R55,D58,R83".split(',').into_iter().collect::<Vec<&str>>());
        let crosses = wire.get_crosses(wire2);
        let mut res = crosses.iter().map(|x| (x.0).2 + (x.1).2).collect::<Vec<i32>>();
        res.sort();
        assert_eq!(res[0], 610);
    }

    #[test]
    fn best_steps_test2() {
        let wire = Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".split(',').into_iter().collect::<Vec<&str>>());
        let wire2 = Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(',').into_iter().collect::<Vec<&str>>());
        let crosses = wire.get_crosses(wire2);
        let mut res = crosses.iter().map(|x| (x.0).2 + (x.1).2).collect::<Vec<i32>>();
        res.sort();
        assert_eq!(res[0], 410);
    }
}
