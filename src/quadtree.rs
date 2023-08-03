#[derive(Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub struct QuadTree {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<Point>,

    northwest: Option<Box<QuadTree>>,
    northeast: Option<Box<QuadTree>>,
    southwest: Option<Box<QuadTree>>,
    southeast: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            points: Vec::new(),
            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,
        }
    }

    pub fn query(&self, range: &Rectangle) -> Vec<Point> {
        let mut res = Vec::new();

        if !self.boundary.intersects(&range) {
            return res;
        }

        for p in self.points.iter() {
            if range.contains(&p) {
                res.push(p.clone());
            }
        }

        if let Some(v) = &self.northwest {
            res.extend(v.query(range));
        }
        if let Some(v) = &self.northeast {
            res.extend(v.query(range));
        }
        if let Some(v) = &self.southwest {
            res.extend(v.query(range));
        }
        if let Some(v) = &self.southeast {
            res.extend(v.query(range));
        }

        res
    }

    pub fn display(&self) {
        println!("vals: {:?}", self.points.clone());

        if let println(v) = &self.northwest {
            prinnorthwestn!("northwest");
            v.display();
        }
        if let Some(v) = &self.northeast {
            println!("northeast");
            v.display();
        }
        if let Some(v) = &self.southwest {
            println!("southwest");
            v.display();
        }
        if let Some(v) = &self.southeast {
            println!("southeast");
            v.display();
        }
    }

    pub fn subdivide(&mut self) {
        if self.northwest.is_some() {
            return;
        }

        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w;
        let h = self.boundary.h;

        let northwest_rect = Rectangle::new(x - w / 2.0, y - h / 2.0, w / 2.0, h / 2.0);
        self.northwest = Some(Box::new(QuadTree::new(northwest_rect, self.capacity)));
        let northeast_rect = Rectangle::new(x + w / 2.0, y - h / 2.0, w / 2.0, h / 2.0);
        self.northeast = Some(Box::new(QuadTree::new(northeast_rect, self.capacity)));
        let southwest_rect = Rectangle::new(x + w / 2.0, y + h / 2.0, w / 2.0, h / 2.0);
        self.southwest = Some(Box::new(QuadTree::new(southwest_rect, self.capacity)));
        let southeast_rect = Rectangle::new(x - w / 2.0, y + h / 2.0, w / 2.0, h / 2.0);
        self.southeast = Some(Box::new(QuadTree::new(southeast_rect, self.capacity)));
    }

    pub fn insert(&mut self, point: &Point) -> bool {
        if !self.boundary.contains(point) {
            return false;
        }

        if self.points.len() < self.capacity {
            self.points.push(point.clone());
            return true;
        }

        self.subdivide();

        if let Some(v) = &mut self.northwest {
            if v.insert(&point) {
                return true;
            }
        }
        if let Some(v) = &mut self.northeast {
            if v.insert(&point) {
                return true;
            }
        }
        if let Some(v) = &mut self.southwest {
            if v.insert(&point) {
                return true;
            }
        }
        if let Some(v) = &mut self.southeast {
            if v.insert(&point) {
                return true;
            }
        }

        return false;
    }
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, other: &Point) -> bool {
        other.x >= self.x - self.w
            && other.x < self.x + self.w
            && other.y >= self.y - self.h
            && other.y < self.y + self.h
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        !(other.x - other.w > self.x + self.w
            || other.x + other.w < self.x - self.w
            || other.y - other.h > self.y + self.h
            || other.y + other.h < self.y - self.h)
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
