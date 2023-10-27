use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Sub};

trait GetArea {
    fn get_area(&self) -> Area;
}

impl Add for &dyn GetArea {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Area { area: self.get_area().area + rhs.get_area().area }
    }
}

#[derive(Debug)]
struct Area {
    area: f32
}
impl Default for Area {
    fn default() -> Self {
        Area { area: 0.0 }
    }
}
impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f , "Area is {} cm²", self.area)
    }
}
impl Add for Area {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Area { area: self.area + rhs.area }
    }
}

impl Sub for Area {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Area { area: self.area - rhs.area }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Circle {
    radius: i32,
    center: Point
}
impl Default for Circle {
    fn default() -> Self {
        Circle {
            radius: 1,
            center: Point { x:0, y:0 }
        }
    }
}
impl GetArea for Circle {
    fn get_area(&self) -> Area {
        Area {
            area: self.radius.pow(2) as f32 * std::f32::consts::PI
        }
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}
impl Default for Rectangle {
    fn default() -> Self {
        Rectangle{
            top_left: Point::new(-1, 1),
            bottom_right: Point::new(1, -1)
        }
    }
}
impl GetArea for Rectangle {
    fn get_area(&self) -> Area {
        let l1 = (self.top_left.x).abs_diff(self.bottom_right.x);
        let l2 = (self.top_left.y).abs_diff(self.bottom_right.y);
        Area {
            area: (l1*l2) as f32
        }
    }
}
impl Add for Rectangle {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Area { area: self.get_area().area + rhs.get_area().area}
    }
}

impl Add for Circle {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Area { area: self.get_area().area + rhs.get_area().area}
    }
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {x, y}
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        Point {
            x: 0,
            y: 0
        }
    }
}
impl GetArea for Point {
    fn get_area(&self) -> Area {
        Area::default()
    }
}


impl <'a> Sum<&'a dyn GetArea > for Area {
    fn sum<I>(iter: I) -> Area
    where
    I: Iterator<Item = &'a dyn GetArea>, {
        iter.map(|shape| shape.get_area())
        .fold(Area {area: 0.0}, |acc, area| Area {area: acc.area + area.area})

    }
}



pub fn es6() {

    let ret1: &dyn GetArea = &Circle::default();
    let ret2: &dyn GetArea = &Rectangle::default();

    let _area = dbg!(ret1 + ret2);
    let aree = vec![ret1, ret2];
    dbg!(sum_area(&aree));

}

fn sum_area(shapes: &[&dyn GetArea]) -> Area {
    let mut area = Area { area: 0.0 }; 
    for shape in shapes.iter() {
        area.area += shape.get_area().area;
    }
    area
}


