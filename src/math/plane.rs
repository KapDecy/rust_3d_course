use crate::triangle::Triangle;

use super::vec_3d::Vec3d;

pub struct Plane {
    normal: Vec3d,
    point: Vec3d,
}

impl Plane {
    pub fn normal(&self) -> &Vec3d {
        &self.normal
    }

    pub fn point(&self) -> &Vec3d {
        &self.point
    }

    pub fn new(normal: Vec3d, point: Vec3d) -> Self {
        Self {
            normal: normal.normalized(),
            point,
        }
    }

    pub fn from_triangle(triangle: &Triangle) -> Self {
        Self {
            normal: triangle.normal(),
            point: Vec3d::from_vec4d(&triangle.points()[0]),
        }
    }

    pub fn distance(&self, point: &Vec3d) -> f64 {
        point.dot(&self.normal) - self.point.dot(&self.normal)
    }

    pub fn intersection(&self, start: &Vec3d, end: &Vec3d) -> (Vec3d, f64) {
        let s_dot_n = start.dot(&self.normal);
        let k = (s_dot_n - self.point.dot(&self.normal)) / (s_dot_n - end.dot(&self.normal));
        let res = start + &((end - start) * k);
        (res, k)
    }

    pub fn clip(&self, tri: &Triangle) -> Vec<Triangle> {
        let mut res = Vec::new();

        let mut inside_points = vec![];
        let mut outside_points = vec![];

        let distances = [
            self.distance(&Vec3d::from_vec4d(&tri.points()[0])),
            self.distance(&Vec3d::from_vec4d(&tri.points()[1])),
            self.distance(&Vec3d::from_vec4d(&tri.points()[2])),
        ];

        for i in 0..3 {
            if distances[i] >= 0.0 {
                inside_points.push(Vec3d::from_vec4d(&tri.points()[i]));
            } else {
                outside_points.push(Vec3d::from_vec4d(&tri.points()[i]));
            }
        }

        if inside_points.len() == 1 {
            let intersect1 = self.intersection(&inside_points[0], &outside_points[0]);
            let intersect2 = self.intersection(&inside_points[0], &outside_points[1]);

            res.push(Triangle::new(
                tri.color(),
                inside_points[0].make_point_4d(),
                intersect1.0.make_point_4d(),
                intersect2.0.make_point_4d(),
            ));
        }

        if inside_points.len() == 2 {
            let intersect1 = self.intersection(&inside_points[0], &outside_points[0]);
            let intersect2 = self.intersection(&inside_points[1], &outside_points[0]);

            res.push(Triangle::new(
                tri.color(),
                inside_points[0].make_point_4d(),
                intersect1.0.make_point_4d(),
                inside_points[1].make_point_4d(),
            ));

            res.push(Triangle::new(
                tri.color(),
                intersect1.0.make_point_4d(),
                intersect2.0.make_point_4d(),
                inside_points[1].make_point_4d(),
            ));
        }

        if inside_points.len() == 3 {
            res.push(tri.clone());
        }

        res
    }
}
