use std::f64::{consts::PI, EPSILON};

#[derive(Clone)]
pub(crate) struct Vec2d {
    x: f64,
    y: f64,
}

impl Vec2d {
    fn new(x: f64, y: f64) -> Vec2d {
        Vec2d { x, y }
    }

    // fn from_vec4d(point4d: &Vec4d) -> Vec2d {
    //     todo!()
    // }

    

    fn sqr_abs(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    fn abs(&self) -> f64 {
        self.sqr_abs().sqrt()
    }

    fn normalized(&self) -> Vec2d {
        if self.abs() > EPSILON {
            self.clone() / self.abs()
        } else {
            Vec2d::new(0.0, 0.0)
        }
    }

    fn dot(&self, rhs: &Vec2d) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl std::ops::Neg for Vec2d {
    type Output = Vec2d;

    fn neg(self) -> Self::Output {
        Vec2d::new(-self.x, -self.y)
    }
}

impl std::ops::Add<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Vec2d) -> Self::Output {
        Vec2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: Vec2d) -> Self::Output {
        Vec2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f64> for Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2d::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<f64> for Vec2d {
    type Output = Vec2d;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs.abs() > EPSILON {
            Vec2d::new(self.x / rhs, self.y / rhs)
        } else {
            panic!("Trying to div by 0")
        }
    }
}

impl PartialEq<Vec2d> for Vec2d {
    fn eq(&self, other: &Vec2d) -> bool {
        // self.x == other.x && self.y == other.y
        let diff = self.clone() - other.clone();

        diff.sqr_abs() < EPSILON
    }
}
