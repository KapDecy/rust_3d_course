use std::f64::consts::PI;

use crate::math::is_near;

use super::vec_3d::Vec3d;
use super::vec_4d::Vec4d;

#[derive(Debug, Clone)]
pub struct Matrix4x4([[f64; 4]; 4]);

impl Matrix4x4 {
    pub fn x(&self) -> Vec3d {
        Vec3d::new(self.0[0][0], self.0[1][0], self.0[2][0])
    }

    pub fn y(&self) -> Vec3d {
        Vec3d::new(self.0[0][1], self.0[1][1], self.0[2][1])
    }

    pub fn z(&self) -> Vec3d {
        Vec3d::new(self.0[0][2], self.0[1][2], self.0[2][2])
    }

    pub fn w(&self) -> Vec3d {
        Vec3d::new(self.0[0][3], self.0[1][3], self.0[2][3])
    }

    fn zero() -> Matrix4x4 {
        Matrix4x4([[0.; 4]; 4])
    }

    pub fn identity() -> Matrix4x4 {
        let mut res = Matrix4x4::zero();

        res.0[0][0] = 1.;
        res.0[1][1] = 1.;
        res.0[2][2] = 1.;
        res.0[3][3] = 1.;

        res
    }

    fn constant(v: f64) -> Matrix4x4 {
        Matrix4x4([[v; 4]; 4])
    }

    pub fn scale(vec: &Vec3d) -> Matrix4x4 {
        let mut res = Matrix4x4::identity();

        res.0[0][0] = vec.x();
        res.0[1][1] = vec.y();
        res.0[2][2] = vec.z();

        res
    }

    pub fn translation(vec: &Vec3d) -> Matrix4x4 {
        let mut res = Matrix4x4::identity();

        res.0[0][3] = vec.x();
        res.0[1][3] = vec.y();
        res.0[2][3] = vec.z();

        res
    }

    fn rotation_x(rx: f64) -> Matrix4x4 {
        let mut res = Matrix4x4::identity();

        let cos = rx.cos();
        let sin = rx.sin();

        res.0[1][1] = cos;
        res.0[1][2] = -sin;
        res.0[2][1] = sin;
        res.0[2][2] = cos;

        res
    }

    fn rotation_y(ry: f64) -> Matrix4x4 {
        let mut res = Matrix4x4::identity();

        let cos = ry.cos();
        let sin = ry.sin();

        res.0[0][0] = cos;
        res.0[0][2] = sin;
        res.0[2][0] = -sin;
        res.0[2][2] = cos;

        res
    }

    fn rotation_z(rz: f64) -> Matrix4x4 {
        let mut res = Matrix4x4::identity();

        let cos = rz.cos();
        let sin = rz.sin();

        res.0[0][0] = cos;
        res.0[0][1] = -sin;
        res.0[1][0] = sin;
        res.0[1][1] = cos;

        res
    }

    pub fn rotation(r: &Vec3d) -> Matrix4x4 {
        Self::rotation_x(r.x()) * Self::rotation_y(r.y()) * Self::rotation_z(r.z())
    }

    pub fn rotation_around_vec(v: &Vec3d, rv: f64) -> Matrix4x4 {
        let mut res = Matrix4x4::identity();

        let nv = v.normalized();

        let c = rv.cos();
        let s = rv.sin();

        res.0[0][0] = c + (1.0 - c) * nv.x() * nv.x();
        res.0[0][1] = (1.0 - c) * nv.x() * nv.y() - s * nv.z();
        res.0[0][2] = (1.0 - c) * nv.x() * nv.z() + s * nv.y();

        res.0[1][0] = (1.0 - c) * nv.x() * nv.y() + s * nv.z();
        res.0[1][1] = c + (1.0 - c) * nv.y() * nv.y();
        res.0[1][2] = (1.0 - c) * nv.y() * nv.z() - s * nv.x();
        res.0[2][0] = (1.0 - c) * nv.z() * nv.x() - s * nv.y();
        res.0[2][1] = (1.0 - c) * nv.z() * nv.y() + s * nv.x();
        res.0[2][2] = c + (1.0 - c) * nv.z() * nv.z();

        res.0[3][3] = 1.;

        res
    }

    pub fn projection(fov: f64, aspect: f64, znear: f64, zfar: f64) -> Matrix4x4 {
        let mut res = Matrix4x4::identity();

        res.0[0][0] = 1. / ((PI * fov * 0.5 / 180.0).tan() * aspect);
        res.0[1][1] = 1. / ((PI * fov * 0.5 / 180.0).tan());
        res.0[2][2] = zfar / (zfar - znear);
        res.0[2][3] = -zfar * znear / (zfar - znear);
        res.0[3][2] = 1.;

        res
    }

    pub fn screen_space(width: i32, height: i32) -> Matrix4x4 {
        let mut res = Matrix4x4::identity();
        res.0[0][0] = -0.5 * width as f64;
        res.0[1][1] = -0.5 * height as f64;
        res.0[2][2] = 1.;

        res.0[0][3] = 0.5 * width as f64;
        res.0[1][3] = 0.5 * height as f64;

        res.0[3][3] = 1.;

        res
    }

    pub fn view(tranform_matrix: &Matrix4x4) -> Matrix4x4 {
        let mut res = Matrix4x4::zero();

        let left = tranform_matrix.x();
        let up = tranform_matrix.y();
        let look_at = tranform_matrix.z();
        let eye = tranform_matrix.w();

        let left_sqr_abs = left.sqr_abs();
        let up_sqr_abs = up.sqr_abs();
        let look_at_sqr_abs = look_at.sqr_abs();

        res.0[0][0] = left.x() / left_sqr_abs;
        res.0[0][1] = left.y() / left_sqr_abs;
        res.0[0][2] = left.z() / left_sqr_abs;
        res.0[0][3] = -eye.dot(&left) / left_sqr_abs;

        res.0[1][0] = up.x() / up_sqr_abs;
        res.0[1][1] = up.y() / up_sqr_abs;
        res.0[1][2] = up.z() / up_sqr_abs;
        res.0[1][3] = -eye.dot(&up) / up_sqr_abs;

        res.0[2][0] = look_at.x() / look_at_sqr_abs;
        res.0[2][1] = look_at.y() / look_at_sqr_abs;
        res.0[2][2] = look_at.z() / look_at_sqr_abs;
        res.0[2][3] = -eye.dot(&look_at) / look_at_sqr_abs;

        res.0[3][3] = 1.;

        res
    }
}

impl std::ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut result = Matrix4x4::zero();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.0[i][j] += self.0[i][k] * rhs.0[k][j];
                }
            }
        }

        result
    }
}

impl std::ops::Mul<Vec4d> for Matrix4x4 {
    type Output = Vec4d;

    fn mul(self, rhs: Vec4d) -> Self::Output {
        Vec4d::new(
            self.0[0][0] * rhs.x() + self.0[0][1] * rhs.y() + self.0[0][2] * rhs.z() + self.0[0][3] * rhs.w(),
            self.0[1][0] * rhs.x() + self.0[1][1] * rhs.y() + self.0[1][2] * rhs.z() + self.0[1][3] * rhs.w(),
            self.0[2][0] * rhs.x() + self.0[2][1] * rhs.y() + self.0[2][2] * rhs.z() + self.0[2][3] * rhs.w(),
            self.0[3][0] * rhs.x() + self.0[3][1] * rhs.y() + self.0[3][2] * rhs.z() + self.0[3][3] * rhs.w(),
        )
    }
}

impl std::ops::Mul<Vec3d> for Matrix4x4 {
    type Output = Vec3d;

    fn mul(self, rhs: Vec3d) -> Self::Output {
        Vec3d::new(
            self.0[0][0] * rhs.x() + self.0[0][1] * rhs.y() + self.0[0][2] * rhs.z(),
            self.0[1][0] * rhs.x() + self.0[1][1] * rhs.y() + self.0[1][2] * rhs.z(),
            self.0[2][0] * rhs.x() + self.0[2][1] * rhs.y() + self.0[2][2] * rhs.z(),
        )
    }
}

#[cfg(test)]
mod Tests {
    use crate::math::is_near;
    use crate::math::vec_3d::Vec3d;
    use crate::math::vec_4d::Vec4d;
    use std::f64::consts::PI;

    use super::Matrix4x4;

    fn matrix_vec_mul() {
        let v = Vec4d::new(4., 2., 3., 1.);

        let one = Matrix4x4::identity() * v.clone();
        assert!(is_near(one.x(), 4.) && is_near(one.y(), 2.) && is_near(one.z(), 3.,) && is_near(one.w(), 1.));

        let scale = Matrix4x4::scale(&Vec3d::new(1., 2., 3.)) * v.clone();
        assert!(is_near(scale.x(), 4.) && is_near(scale.y(), 4.) && is_near(scale.z(), 9.,) && is_near(scale.w(), 1.));

        let zero = Matrix4x4::zero() * v.clone();
        assert!(is_near(zero.x(), 0.) && is_near(zero.y(), 0.) && is_near(zero.z(), 0.) && is_near(zero.w(), 0.));

        let trans = Matrix4x4::translation(&Vec3d::new(5., 4., 3.)) * v;
        assert!(is_near(trans.x(), 9.) && is_near(trans.y(), 6.) && is_near(trans.z(), 6.) && is_near(trans.w(), 1.));
    }

    fn matrix_matrix_mul() {
        let c1 = Matrix4x4::constant(5.) * Matrix4x4::identity();
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(c1.0[i][j], 5.);
            }
        }

        let c2 = Matrix4x4::zero() * Matrix4x4::constant(1.);
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(c2.0[i][j], 0.);
            }
        }

        let c3 = Matrix4x4::identity() * Matrix4x4::scale(&Vec3d::new(3., 3., 3.));
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    if i < 3 {
                        assert_eq!(c3.0[i][j], 3.);
                    } else {
                        assert_eq!(c3.0[i][j], 1.);
                    }
                } else {
                    assert_eq!(c3.0[i][j], 0.);
                }
            }
        }
    }

    fn rotation() {
        let i = Vec4d::new(1., 0., 0., 0.);
        let j = Vec4d::new(0., 1., 0., 0.);
        let k = Vec4d::new(0., 0., 1., 0.);

        let i_rx = Matrix4x4::rotation_x(PI / 2.) * i.clone();
        let j_rx = Matrix4x4::rotation_x(PI / 2.) * j.clone();
        let k_rx = Matrix4x4::rotation_x(PI / 2.) * k.clone();

        assert_eq!(i_rx, i);
        assert_eq!(j_rx, k);
        assert_eq!(k_rx, -j.clone());

        let i_ry = Matrix4x4::rotation_y(PI / 2.) * i.clone();
        let j_ry = Matrix4x4::rotation_y(PI / 2.) * j.clone();
        let k_ry = Matrix4x4::rotation_y(PI / 2.) * k.clone();

        assert_eq!(i_ry, -k.clone());
        assert_eq!(j_ry, j);
        assert_eq!(k_ry, i);

        let i_rz = Matrix4x4::rotation_z(PI / 2.) * i.clone();
        let j_rz = Matrix4x4::rotation_z(PI / 2.) * j.clone();
        let k_rz = Matrix4x4::rotation_z(PI / 2.) * k.clone();

        assert_eq!(i_rz, j);
        assert_eq!(j_rz, -i);
        assert_eq!(k_rz, k);
    }
}
