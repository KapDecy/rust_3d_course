use std::f64::EPSILON;

use rand::random;

use super::vec_4d::Vec4d;

#[derive(Clone, Debug)]
pub struct Vec3d([f64; 3]);

impl Vec3d {
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d([x, y, z])
    }

    pub fn sqr_abs(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    fn abs(&self) -> f64 {
        self.sqr_abs().sqrt()
    }

    pub fn normalized(&self) -> Vec3d {
        if self.abs() > EPSILON {
            self.clone() / self.abs()
        } else {
            Vec3d::new(0.0, 0.0, 0.0)
        }
    }

    pub fn dot(&self, rhs: &Vec3d) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vec3d) -> Vec3d {
        Vec3d([
            self.y() * rhs.z() - rhs.y() * self.z(),
            self.z() * rhs.x() - rhs.z() * self.x(),
            self.x() * rhs.y() - rhs.x() * self.y(),
        ])
    }

    pub fn from_vec4d(v: &Vec4d) -> Vec3d {
        Vec3d([v.x(), v.y(), v.z()])
    }

    pub fn make_point_4d(&self) -> Vec4d {
        Vec4d::new(self.x(), self.y(), self.z(), 1.0)
    }

    fn random() -> Vec3d {
        Vec3d::new(
            random::<f64>() / f64::MAX,
            random::<f64>() / f64::MAX,
            random::<f64>() / f64::MAX,
        )
    }
}

impl std::ops::Neg for Vec3d {
    type Output = Vec3d;

    fn neg(self) -> Self::Output {
        Vec3d::new(-self.x(), -self.y(), -self.z())
    }
}

impl std::ops::Add<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn add(self, rhs: Vec3d) -> Self::Output {
        Vec3d::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl std::ops::Sub<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn sub(self, rhs: Vec3d) -> Self::Output {
        Vec3d::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl std::ops::Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3d::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl std::ops::Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs.abs() > EPSILON {
            Vec3d::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
        } else {
            panic!("Trying to div by 0")
        }
    }
}

impl PartialEq<Vec3d> for Vec3d {
    fn eq(&self, other: &Vec3d) -> bool {
        // self.x() == other.x && self.y() == other.y
        let diff = self.clone() - other.clone();

        diff.sqr_abs() < EPSILON
    }
}

// borrow

impl std::ops::Neg for &Vec3d {
    type Output = Vec3d;

    fn neg(self) -> Self::Output {
        Vec3d::new(-self.x(), -self.y(), -self.z())
    }
}

impl std::ops::Add<&Vec3d> for &Vec3d {
    type Output = Vec3d;

    fn add(self, rhs: &Vec3d) -> Self::Output {
        Vec3d::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl std::ops::Sub<&Vec3d> for &Vec3d {
    type Output = Vec3d;

    fn sub(self, rhs: &Vec3d) -> Self::Output {
        Vec3d::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl std::ops::Mul<f64> for &Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3d::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl std::ops::Div<f64> for &Vec3d {
    type Output = Vec3d;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs.abs() > EPSILON {
            Vec3d::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
        } else {
            panic!("Trying to div by 0")
        }
    }
}

#[cfg(test)]
mod Tests {
    use crate::math::is_near;

    use super::Vec3d;

    #[test]
    fn copy() {
        let a = Vec3d::new(1., 2., 3.);
        let b = Vec3d::new(3., 4., 5.);
        let c = a.clone();
        assert!(is_near(c.x(), 1.) && is_near(c.y(), 2.) && is_near(c.z(), 3.))
    }

    #[test]
    fn assign() {
        let a = Vec3d::new(1., 2., 3.);
        let b = Vec3d::new(3., 4., 5.);
        let c = b;
        assert!(is_near(c.x(), 3.) && is_near(c.y(), 4.) && is_near(c.z(), 5.))
    }

    #[test]
    fn neg() {
        let a = Vec3d::new(1., 2., 3.);
        let b = Vec3d::new(3., 4., 5.);
        let neg = -a;
        assert!(is_near(neg.x(), -1.) && is_near(neg.y(), -2.) && is_near(neg.z(), -3.))
    }

    #[test]
    fn plus_minus() {
        let a = Vec3d::new(1., 2., 3.);
        let b = Vec3d::new(3., 4., 5.);
        let summ = a.clone() + b.clone();
        let diff = a - b;
        assert!(is_near(summ.x(), 4.) && is_near(summ.y(), 6.) && is_near(summ.z(), 8.));
        assert!(is_near(diff.x(), -2.) && is_near(diff.y(), -2.) && is_near(diff.z(), -2.));
    }

    #[test]
    fn eq() {
        let a = Vec3d::new(1., 2., 3.);
        let b = Vec3d::new(3., 4., 5.);
        let c = b.clone();
        assert!(c != a && c == b)
    }

    #[test]
    fn scale() {
        let a = Vec3d::new(1., 2., 3.);
        let b = Vec3d::new(3., 4., 5.);
        let scale1 = a.clone() * 2.;
        assert!(is_near(scale1.x(), 2.) && is_near(scale1.y(), 4.) && is_near(scale1.z(), 6.));
        let scale2 = a.clone() / 2.;
        assert!(is_near(scale2.x(), 0.5) && is_near(scale2.y(), 1.) && is_near(scale2.z(), 1.5));
    }

    #[test]
    fn dot() {
        let a = Vec3d::new(1., 2., 3.);
        let b = Vec3d::new(3., 4., 5.);
        assert!(is_near(a.dot(&b), 26.))
    }

    #[test]
    fn cross() {
        let a = Vec3d::new(1., 2., 3.);
        let b = Vec3d::new(3., 4., 5.);
        let cross = a.cross(&b);
        let neg_cross = b.cross(&a);
        assert!(is_near(a.dot(&cross), 0.) && is_near(b.dot(&cross), 0.));
        assert_eq!(-cross, neg_cross)
    }

    #[test]
    fn abs_normalized() {
        let a = Vec3d::new(1., 2., 3.);
        let b = Vec3d::new(3., 4., 5.);
        assert!(is_near(b.abs(), 50.0_f64.sqrt()));
        assert!(is_near(b.normalized().abs(), 1.));
    }
}
