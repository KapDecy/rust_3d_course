use std::f64::EPSILON;

#[derive(Debug, Clone)]
pub struct Vec4d([f64; 4]);

impl Vec4d {
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }
    pub fn w(&self) -> f64 {
        self.0[3]
    }
    fn sqr_abs(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w()
    }

    fn abs(&self) -> f64 {
        self.sqr_abs().sqrt()
    }

    fn normalized(&self) -> Vec4d {
        if self.abs() > EPSILON {
            self / self.abs()
        } else {
            Vec4d::new(0.0, 0.0, 0.0, 0.0)
        }
    }
}

impl std::ops::Neg for &Vec4d {
    type Output = Vec4d;

    fn neg(self) -> Self::Output {
        Vec4d::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl std::ops::Add<&Vec4d> for &Vec4d {
    type Output = Vec4d;

    fn add(self, rhs: &Vec4d) -> Self::Output {
        Vec4d::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z(), self.w() + rhs.w())
    }
}

impl std::ops::Sub<&Vec4d> for &Vec4d {
    type Output = Vec4d;

    fn sub(self, rhs: &Vec4d) -> Self::Output {
        Vec4d::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z(), self.w() - rhs.w())
    }
}

impl std::ops::Mul<f64> for &Vec4d {
    type Output = Vec4d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec4d::new(self.x() * rhs, self.y() * rhs, self.z() * rhs, self.w() * rhs)
    }
}

impl std::ops::Div<f64> for &Vec4d {
    type Output = Vec4d;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs.abs() > EPSILON {
            Vec4d::new(self.x() / rhs, self.y() / rhs, self.z() / rhs, self.w() / rhs)
        } else {
            panic!("Trying to div by 0")
        }
    }
}

impl PartialEq<Vec4d> for Vec4d {
    fn eq(&self, other: &Vec4d) -> bool {
        // self.x() == other.x() && self.y() == other.y()
        let diff = self - other;

        diff.sqr_abs() < EPSILON
    }
}

// owned

impl std::ops::Neg for Vec4d {
    type Output = Vec4d;

    fn neg(self) -> Self::Output {
        Vec4d::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl std::ops::Add<&Vec4d> for Vec4d {
    type Output = Vec4d;

    fn add(self, rhs: &Vec4d) -> Self::Output {
        Vec4d::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z(), self.w() + rhs.w())
    }
}

impl std::ops::Sub<&Vec4d> for Vec4d {
    type Output = Vec4d;

    fn sub(self, rhs: &Vec4d) -> Self::Output {
        Vec4d::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z(), self.w() - rhs.w())
    }
}

impl std::ops::Mul<f64> for Vec4d {
    type Output = Vec4d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec4d::new(self.x() * rhs, self.y() * rhs, self.z() * rhs, self.w() * rhs)
    }
}

impl std::ops::Div<f64> for Vec4d {
    type Output = Vec4d;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs.abs() > EPSILON {
            Vec4d::new(self.x() / rhs, self.y() / rhs, self.z() / rhs, self.w() / rhs)
        } else {
            panic!("Trying to div by 0")
        }
    }
}

impl Vec4d {
    pub(crate) fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4d {
        Vec4d([x, y, z, w])
    }
}

#[cfg(test)]
mod Tests {
    use crate::math::is_near;

    use super::Vec4d;

    #[test]
    fn copy() {
        let a = Vec4d::new(1., 2., 3., 4.);
        let b = Vec4d::new(3., 4., 5., 6.);
        let c = a.clone();
        assert!(is_near(c.x(), 1.) && is_near(c.y(), 2.) && is_near(c.z(), 3.) && is_near(c.w(), 4.))
    }

    #[test]
    fn assign() {
        let a = Vec4d::new(1., 2., 3., 4.);
        let b = Vec4d::new(3., 4., 5., 6.);
        let c = b;
        assert!(is_near(c.x(), 3.) && is_near(c.y(), 4.) && is_near(c.z(), 5.) && is_near(c.w(), 6.))
    }

    #[test]
    fn neg() {
        let a = Vec4d::new(1., 2., 3., 4.);
        let b = Vec4d::new(3., 4., 5., 6.);
        let neg = -a;
        assert!(is_near(neg.x(), -1.) && is_near(neg.y(), -2.) && is_near(neg.z(), -3.) && is_near(neg.w(), -4.))
    }

    #[test]
    fn plus_minus() {
        let a = Vec4d::new(1., 2., 3., 4.);
        let b = Vec4d::new(3., 4., 5., 6.);
        let summ = &a + &b;
        let diff = &a - &b;
        assert!(is_near(summ.x(), 4.) && is_near(summ.y(), 6.) && is_near(summ.z(), 8.) && is_near(summ.w(), 10.));
        assert!(is_near(diff.x(), -2.) && is_near(diff.y(), -2.) && is_near(diff.z(), -2.) && is_near(diff.w(), -2.));
    }

    #[test]
    fn eq() {
        let a = Vec4d::new(1., 2., 3., 4.);
        let b = Vec4d::new(3., 4., 5., 6.);
        let c = b.clone();
        assert!(c != a && c == b)
    }

    #[test]
    fn scale() {
        let a = Vec4d::new(1., 2., 3., 4.);
        let b = Vec4d::new(3., 4., 5., 6.);
        let scale1 = a.clone() * 2.;
        assert!(is_near(scale1.x(), 2.) && is_near(scale1.y(), 4.) && is_near(scale1.z(), 6.) && is_near(scale1.w(), 8.));
        let scale2 = a.clone() / 2.;
        assert!(is_near(scale2.x(), 0.5) && is_near(scale2.y(), 1.) && is_near(scale2.z(), 1.5) && is_near(scale2.w(), 2.));
    }

    #[test]
    fn abs_normalized() {
        let a = Vec4d::new(1., 2., 3., 4.);
        let b = Vec4d::new(3., 4., 5., 6.);
        assert!(is_near(b.abs(), 86.0_f64.sqrt()));
        assert!(is_near(b.normalized().abs(), 1.));
    }
}
