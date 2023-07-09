use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::rc::{self, Rc};

use macroquad::prelude::Color;

use crate::math::matrix4x4::Matrix4x4;
use crate::math::plane::Plane;
use crate::math::vec_3d::Vec3d;
use crate::object::{Mesh, Object, ObjectNameTag, ObjectStruct};
use crate::triangle::Triangle;

pub struct MyCamera {
    obj: ObjectStruct,
    triangles: Vec<Triangle>,
    clip_planes: Vec<Plane>,
    aspect: f64,
    sp: Matrix4x4,
}

impl MyCamera {
    pub fn new(name: &str) -> Self {
        Self {
            obj: ObjectStruct::new(ObjectNameTag::new(name)),
            triangles: vec![],
            clip_planes: vec![],
            aspect: 0.0,
            sp: Matrix4x4::identity(),
        }
    }

    pub fn init(&mut self, width: i32, height: i32, fov: f64, znear: f64, zfar: f64) {
        self.aspect = width as f64 / height as f64;
        let p = Matrix4x4::projection(fov, self.aspect, znear, zfar);
        let s = Matrix4x4::screen_space(width, height);
        self.sp = s * p;

        self.clip_planes
            .push(Plane::new(Vec3d::new(0.0, 0.0, 1.0), Vec3d::new(0.0, 0.0, znear)));
        self.clip_planes
            .push(Plane::new(Vec3d::new(0.0, 0.0, -1.0), Vec3d::new(0.0, 0.0, zfar)));

        let thetta1 = PI * fov * 0.5 / 180.0;
        let thetta2 = (self.aspect * thetta1.tan()).atan();

        self.clip_planes.push(Plane::new(
            Vec3d::new(-thetta2.cos(), 0.0, thetta2.sin()),
            Vec3d::new(0.0, 0.0, 0.0),
        ));
        self.clip_planes.push(Plane::new(
            Vec3d::new(thetta2.cos(), 0.0, thetta2.sin()),
            Vec3d::new(0.0, 0.0, 0.0),
        ));
        self.clip_planes.push(Plane::new(
            Vec3d::new(0.0, thetta1.cos(), thetta1.sin()),
            Vec3d::new(0.0, 0.0, 0.0),
        ));
        self.clip_planes.push(Plane::new(
            Vec3d::new(0.0, -thetta1.cos(), thetta1.sin()),
            Vec3d::new(0.0, 0.0, 0.0),
        ));
    }

    pub fn sorted(&mut self) -> &Vec<Triangle> {
        self.triangles.sort_by(|t1, t2| {
            let mut v_z1 = vec![t1.points()[0].z(), t1.points()[1].z(), t1.points()[2].z()];
            let mut v_z2 = vec![t2.points()[0].z(), t2.points()[1].z(), t2.points()[2].z()];

            v_z1.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v_z2.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let z1 = v_z1[0] + v_z1[1] + v_z1[2];
            let z2 = v_z2[0] + v_z2[1] + v_z2[2];

            return z1.total_cmp(&z2);
        });
        &self.triangles
    }

    pub fn project(&mut self, mesh: Rc<RefCell<dyn Mesh>>) -> Vec<Triangle> {
        let mesh = mesh.borrow();

        if !mesh.visible() {
            return vec![];
        }

        let m = mesh.model();
        let v = self.inv_model();

        let mut clipped_triangles: Vec<Triangle> = vec![];
        let mut temp_buffer: Vec<Triangle> = vec![];

        for t in mesh.triangles() {
            let m_tri = t * &m;

            let dot = m_tri
                .normal()
                .dot(&(&Vec3d::from_vec4d(&m_tri.points()[0]) - self.position()).normalized());
            // TODO: enable it
            // if dot >= 0.0 {
            //     continue;
            // }

            let vm_tri = &m_tri * &v;

            clipped_triangles.clear();
            temp_buffer.clear();

            clipped_triangles.push(vm_tri);

            for plane in &self.clip_planes {
                while !clipped_triangles.is_empty() {
                    let clip_result = plane.clip(clipped_triangles.last().unwrap());
                    clipped_triangles.pop();
                    for i in clip_result {
                        temp_buffer.push(i);
                    }
                }
                std::mem::swap(&mut clipped_triangles, &mut temp_buffer);
            }

            for clipped in clipped_triangles.iter() {
                let color = clipped.color();
                let ambient_color = Color::from_rgba(
                    ((color.r * (0.3 * dot.abs() + 0.7) as f32) * 255.0) as u8,
                    ((color.g * (0.3 * dot.abs() + 0.7) as f32) * 255.0) as u8,
                    ((color.b * (0.3 * dot.abs() + 0.7) as f32) * 255.0) as u8,
                    (color.a * 255.0) as u8,
                );
                let clipped_projected = clipped * &self.sp;

                let cp_points = clipped_projected.points();
                let clip_proj_norm = Triangle::new(
                    // ambient_color,
                    ambient_color,
                    cp_points[0].clone() / cp_points[0].w(),
                    cp_points[1].clone() / cp_points[1].w(),
                    cp_points[2].clone() / cp_points[2].w(),
                );
                self.triangles.push(clip_proj_norm);
            }
        }

        return self.triangles.clone();
    }

    fn buffsize(&self) -> usize {
        self.triangles.len()
    }

    pub fn clear(&mut self) {
        self.triangles.clear();
    }
}

impl Object for MyCamera {
    fn nametag(&self) -> &ObjectNameTag {
        &self.obj.nametag
    }
    fn nametag_mut(&mut self) -> &mut ObjectNameTag {
        &mut self.obj.nametag
    }
    fn transform_matrix(&self) -> &Matrix4x4 {
        &self.obj.transform
    }
    fn transform_matrix_mut(&mut self) -> &mut Matrix4x4 {
        &mut self.obj.transform
    }
    fn position(&self) -> &Vec3d {
        &self.obj.position
    }
    fn position_mut(&mut self) -> &mut Vec3d {
        &mut self.obj.position
    }
    fn angle(&self) -> &Vec3d {
        &self.obj.angle
    }
    fn angle_mut(&mut self) -> &mut Vec3d {
        &mut self.obj.angle
    }
    fn angle_left_up_look_at(&self) -> &Vec3d {
        &self.obj.angle_left_up_look_at
    }
    fn angle_left_up_look_at_mut(&mut self) -> &mut Vec3d {
        &mut self.obj.angle_left_up_look_at
    }
    fn attached_objects(&self) -> &HashMap<ObjectNameTag, rc::Weak<RefCell<dyn Object>>> {
        &self.obj.attached_objects
    }
    fn attached_objects_mut(&mut self) -> &mut HashMap<ObjectNameTag, rc::Weak<RefCell<dyn Object>>> {
        &mut self.obj.attached_objects
    }
}
