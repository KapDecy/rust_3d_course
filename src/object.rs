use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{self, Rc};

use macroquad::prelude::Color;

use crate::math::matrix4x4::Matrix4x4;
use crate::math::vec_3d::Vec3d;
use crate::math::vec_4d::Vec4d;
use crate::triangle::Triangle;

#[derive(Debug, Eq, Hash, Clone)]
pub struct ObjectNameTag(String);

impl ObjectNameTag {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn contains(&self, name_tag: &str) -> bool {
        self.0.contains(name_tag)
    }
}

impl PartialEq for ObjectNameTag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for ObjectNameTag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

pub trait Object {
    fn nametag(&self) -> &ObjectNameTag;
    fn nametag_mut(&mut self) -> &mut ObjectNameTag;
    fn transform_matrix(&self) -> &Matrix4x4;
    fn transform_matrix_mut(&mut self) -> &mut Matrix4x4;
    fn position(&self) -> &Vec3d;
    fn position_mut(&mut self) -> &mut Vec3d;
    fn angle(&self) -> &Vec3d;
    fn angle_mut(&mut self) -> &mut Vec3d;
    fn angle_left_up_look_at(&self) -> &Vec3d;
    fn angle_left_up_look_at_mut(&mut self) -> &mut Vec3d;
    fn attached_objects(&self) -> &HashMap<ObjectNameTag, rc::Weak<RefCell<dyn Object>>>;
    fn attached_objects_mut(&mut self) -> &mut HashMap<ObjectNameTag, rc::Weak<RefCell<dyn Object>>>;

    fn left(&self) -> Vec3d {
        self.transform_matrix().x().normalized()
    }

    fn up(&self) -> Vec3d {
        self.transform_matrix().y().normalized()
    }

    fn look_at(&self) -> Vec3d {
        self.transform_matrix().z().normalized()
    }

    fn transform(&mut self, matrix: &Matrix4x4) {
        *self.transform_matrix_mut() = self.transform_matrix().clone() * matrix.clone();
        let position = self.position().clone();

        for (_, object) in self.attached_objects_mut() {
            if let Some(o) = object.upgrade() {
                o.borrow_mut().transform_relative_point(&position, matrix);
            }
        }
    }

    fn transform_relative_point(&mut self, point: &Vec3d, transform: &Matrix4x4) {
        // translate object in new coordinate system (connected with point)
        *self.transform_matrix_mut() = Matrix4x4::translation(&(self.position().clone() - point.clone())) * transform.clone();
        // transform object in the new coordinate system
        *self.transform_matrix_mut() = transform.clone() * self.transform_matrix().clone();
        // translate object back in self connected coordinate system
        *self.position_mut() = self.transform_matrix().w() + point.clone();
        *self.transform_matrix_mut() = Matrix4x4::translation(&-self.transform_matrix().w()) * self.transform_matrix().clone();

        for (_, object) in self.attached_objects_mut() {
            if let Some(o) = object.upgrade() {
                o.borrow_mut().transform_relative_point(point, transform);
            }
        }
    }

    fn translate(&mut self, dv: &Vec3d) {
        let pos = self.position();
        *self.position_mut() = pos.clone() + dv.clone();

        for (_, object) in self.attached_objects_mut() {
            if let Some(o) = object.upgrade() {
                o.borrow_mut().translate(dv)
            }
        }
    }

    fn scale(&mut self, s: &Vec3d) {
        self.transform(&Matrix4x4::scale(s));
    }

    fn rotate(&mut self, r: &Vec3d) {
        *self.angle_mut() = self.angle().clone() + r.clone();

        self.transform(&Matrix4x4::rotation(r));
    }

    fn rotate_around_vec(&mut self, v: &Vec3d, rv: f64) {
        self.transform(&Matrix4x4::rotation_around_vec(v, rv));
    }

    fn rotate_relative_point(&mut self, s: &Vec3d, r: &Vec3d) {
        *self.angle_mut() = self.angle().clone() + r.clone();

        self.transform_relative_point(s, &Matrix4x4::rotation(r));
    }

    fn rotate_relative_point_around_vec(&mut self, s: &Vec3d, v: &Vec3d, r: f64) {
        self.transform_relative_point(s, &Matrix4x4::rotation_around_vec(v, r));
    }

    fn rotate_left(&mut self, rl: f64) {
        *self.angle_left_up_look_at_mut() = Vec3d::new(
            self.angle_left_up_look_at().x() + rl,
            self.angle_left_up_look_at().y(),
            self.angle_left_up_look_at().z(),
        );

        self.rotate_around_vec(&self.left(), rl)
    }

    fn rotate_up(&mut self, ru: f64) {
        *self.angle_left_up_look_at_mut() = Vec3d::new(
            self.angle_left_up_look_at().x(),
            self.angle_left_up_look_at().y() + ru,
            self.angle_left_up_look_at().z(),
        );

        self.rotate_around_vec(&self.up(), ru)
    }

    fn rotate_look_at(&mut self, rla: f64) {
        *self.angle_left_up_look_at_mut() = Vec3d::new(
            self.angle_left_up_look_at().x(),
            self.angle_left_up_look_at().y(),
            self.angle_left_up_look_at().z() + rla,
        );

        self.rotate_around_vec(&self.look_at(), rla)
    }

    fn translate_to_point(&mut self, point: &Vec3d) {
        self.translate(&(point.clone() - self.position().clone()));
    }

    fn attract_to_point(&mut self, point: &Vec3d, value: f64) {
        let v = (point - self.position()).normalized();
        self.translate(&(&v * value));
    }

    fn rotate_to_angle(&mut self, angle: &Vec3d) {
        self.rotate(&(angle - self.angle()));
    }

    fn attached(&self, tag: &ObjectNameTag) -> Option<Rc<RefCell<dyn Object>>> {
        let obj = self.attached_objects().get(tag);
        if let Some(obj) = obj {
            obj.upgrade()
        } else {
            None
        }
    }

    fn check_if_attached(&self, tag: &ObjectNameTag) -> bool {
        for (obj_tag, obj) in self.attached_objects() {
            if obj_tag == tag {
                return true;
            }
            if let Some(o) = obj.upgrade() {
                if o.borrow().check_if_attached(tag) {
                    return true;
                }
            }
        }
        false
    }

    fn attach(&mut self, obj: Rc<RefCell<dyn Object>>) {
        if self.nametag() != obj.borrow().nametag() {
            if !obj.borrow().check_if_attached(self.nametag()) {
                self.attached_objects_mut()
                    .insert(obj.borrow().nametag().clone(), Rc::downgrade(&obj));
            } else {
                panic!("Object::attach: You tried to create infinite recursive call chains")
            }
        } else {
            panic!("Object::attach: You cannot attach object to itself")
        }
    }

    fn unattach(&mut self, tag: &ObjectNameTag) {
        self.attached_objects_mut().remove(tag);
    }

    fn model(&self) -> Matrix4x4 {
        Matrix4x4::translation(self.position()) * self.transform_matrix().clone()
    }

    fn inv_model(&self) -> Matrix4x4 {
        Matrix4x4::view(&self.model())
    }
}

pub trait Mesh: Object {
    fn color(&self) -> Color;
    fn color_mut(&mut self) -> &mut Color;
    fn triangles(&self) -> &Vec<Triangle>;
    fn triangles_mut(&mut self) -> &mut Vec<Triangle>;
    fn visible(&self) -> bool;
    fn visible_mut(&mut self) -> &mut bool;
}

pub struct ObjectStruct {
    pub nametag: ObjectNameTag,
    pub transform: Matrix4x4,
    pub position: Vec3d,
    pub angle: Vec3d,
    pub angle_left_up_look_at: Vec3d,
    pub attached_objects: HashMap<ObjectNameTag, rc::Weak<RefCell<dyn Object>>>,
}

impl ObjectStruct {
    pub fn new(nametag: ObjectNameTag) -> Self {
        ObjectStruct {
            nametag,
            transform: Matrix4x4::identity(),
            position: Vec3d::new(0.0, 0.0, 0.0),
            angle: Vec3d::new(0.0, 0.0, 0.0),
            angle_left_up_look_at: Vec3d::new(0.0, 0.0, 0.0),
            attached_objects: HashMap::new(),
        }
    }
}

pub struct Cube {
    obj: ObjectStruct,
    color: Color,
    pub triangles: Vec<Triangle>,
    visible: bool,
}

impl Object for Cube {
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

impl Mesh for Cube {
    fn color(&self) -> Color {
        self.color
    }
    fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }
    fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
    fn triangles_mut(&mut self) -> &mut Vec<Triangle> {
        &mut self.triangles
    }
    fn visible(&self) -> bool {
        self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
}

impl std::ops::MulAssign<&Matrix4x4> for Cube {
    fn mul_assign(&mut self, rhs: &Matrix4x4) {
        let mut new_tries = Vec::with_capacity(self.triangles().len());
        for tri in self.triangles() {
            new_tries.push(tri * rhs);
        }
        *self.triangles_mut() = new_tries;
    }
}

impl Cube {
    pub fn new(nametag: ObjectNameTag, size: f64, color: Color) -> Cube {
        let obj = ObjectStruct {
            nametag,
            transform: Matrix4x4::identity(),
            position: Vec3d::new(0.0, 0.0, 0.0),
            angle: Vec3d::new(0.0, 0.0, 0.0),
            angle_left_up_look_at: Vec3d::new(0.0, 0.0, 0.0),
            attached_objects: HashMap::new(),
        };

        let mut cube = Cube {
            obj,
            color,
            triangles: vec![],
            visible: true,
        };

        // 1
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(0.0, 0.0, 0.0, 1.0),
            Vec4d::new(0.0, 1.0, 0.0, 1.0),
            Vec4d::new(1.0, 1.0, 0.0, 1.0),
        ));
        // 2
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(0.0, 0.0, 0.0, 1.0),
            Vec4d::new(1.0, 1.0, 0.0, 1.0),
            Vec4d::new(1.0, 0.0, 0.0, 1.0),
        ));
        // 3
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(1.0, 0.0, 0.0, 1.0),
            Vec4d::new(1.0, 1.0, 0.0, 1.0),
            Vec4d::new(1.0, 1.0, 1.0, 1.0),
        ));
        // 4
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(1.0, 0.0, 0.0, 1.0),
            Vec4d::new(1.0, 1.0, 1.0, 1.0),
            Vec4d::new(1.0, 0.0, 1.0, 1.0),
        ));
        // 5
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(1.0, 0.0, 1.0, 1.0),
            Vec4d::new(1.0, 1.0, 1.0, 1.0),
            Vec4d::new(0.0, 1.0, 1.0, 1.0),
        ));
        // 6
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(1.0, 0.0, 1.0, 1.0),
            Vec4d::new(0.0, 1.0, 1.0, 1.0),
            Vec4d::new(0.0, 0.0, 1.0, 1.0),
        ));
        // 7
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(0.0, 0.0, 1.0, 1.0),
            Vec4d::new(0.0, 1.0, 1.0, 1.0),
            Vec4d::new(0.0, 1.0, 0.0, 1.0),
        ));
        // 8
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(0.0, 0.0, 1.0, 1.0),
            Vec4d::new(0.0, 1.0, 0.0, 1.0),
            Vec4d::new(0.0, 0.0, 0.0, 1.0),
        ));
        // 9
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(0.0, 1.0, 0.0, 1.0),
            Vec4d::new(0.0, 1.0, 1.0, 1.0),
            Vec4d::new(1.0, 1.0, 1.0, 1.0),
        ));
        // 10
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(0.0, 1.0, 0.0, 1.0),
            Vec4d::new(1.0, 1.0, 1.0, 1.0),
            Vec4d::new(1.0, 1.0, 0.0, 1.0),
        ));
        // 11
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(1.0, 0.0, 1.0, 1.0),
            Vec4d::new(0.0, 0.0, 1.0, 1.0),
            Vec4d::new(0.0, 0.0, 0.0, 1.0),
        ));
        // 12
        cube.triangles.push(Triangle::new(
            color,
            Vec4d::new(1.0, 0.0, 1.0, 1.0),
            Vec4d::new(0.0, 0.0, 0.0, 1.0),
            Vec4d::new(1.0, 0.0, 0.0, 1.0),
        ));

        let m = Matrix4x4::scale(&Vec3d::new(size, size, size)) * Matrix4x4::translation(&Vec3d::new(-0.5, -0.5, -0.5));
        cube *= &m;

        cube
    }
}
