use std::f64::EPSILON;
use std::ops;

use macroquad::prelude::Color;

use crate::math::matrix4x4::Matrix4x4;
use crate::math::vec_3d::Vec3d;
use crate::math::vec_4d::Vec4d;

#[derive(Debug, Clone)]
pub struct Triangle {
    color: Color,
    points: [Vec4d; 3],
    normal: Vec3d,
}

impl Triangle {
    pub fn points(&self) -> &[Vec4d; 3] {
        &self.points
    }

    pub fn new(color: Color, p1: Vec4d, p2: Vec4d, p3: Vec4d) -> Self {
        let arr = [p1, p2, p3];
        let norm = Self::calculate_normal(&arr);
        Self {
            color,
            points: arr,
            normal: norm,
        }
    }
    pub fn calculate_normal(points: &[Vec4d; 3]) -> Vec3d {
        let v1 = Vec3d::from_vec4d(&(&points[1] - &points[0]));
        let v2 = Vec3d::from_vec4d(&(&points[2] - &points[0]));
        let cross = v1.cross(&v2);
        if cross.sqr_abs() > EPSILON {
            cross.normalized()
        } else {
            Vec3d::new(0.0, 0.0, 0.0)
        }
    }

    pub fn normal(&self) -> Vec3d {
        self.normal.clone()
    }

    fn is_point_inside(&self, point: &Vec3d) -> bool {
        let tri_normal = self.normal();
        let dot1 = (point.clone() - Vec3d::from_vec4d(&self.points[0]))
            .cross(&Vec3d::from_vec4d(&(&self.points[1] - &self.points[0])))
            .dot(&tri_normal);
        let dot2 = (point.clone() - Vec3d::from_vec4d(&self.points[1]))
            .cross(&Vec3d::from_vec4d(&(&self.points[2] - &self.points[1])))
            .dot(&tri_normal);
        let dot3 = (point.clone() - Vec3d::from_vec4d(&self.points[2]))
            .cross(&Vec3d::from_vec4d(&(&self.points[0] - &self.points[2])))
            .dot(&tri_normal);

        if (dot1 >= 0.0 && dot2 >= 0.0 && dot3 >= 0.0) || (dot1 <= 0.0 && dot2 <= 0.0 && dot3 <= 0.0) {
            return true;
        }
        false
    }

    pub fn distance(&self, point: &Vec3d) -> f64 {
        self.normal().dot(&(Vec3d::from_vec4d(&self.points[0]) - point.clone()))
    }

    pub fn color(&self) -> Color {
        self.color
    }
    pub fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    pub fn position(&self) -> Vec3d {
        Vec3d::from_vec4d(&((&self.points[0] + &self.points[1] + &self.points[2]) / 3.0))
    }
}

impl ops::Mul<&Matrix4x4> for &Triangle {
    type Output = Triangle;

    fn mul(self, rhs: &Matrix4x4) -> Self::Output {
        Triangle::new(
            self.color,
            rhs.clone() * self.points[0].clone(),
            rhs.clone() * self.points[1].clone(),
            rhs.clone() * self.points[2].clone(),
        )
    }
}
