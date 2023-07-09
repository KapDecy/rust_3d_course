use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;
use rust_3d_course::camera::MyCamera;
use rust_3d_course::math::vec_3d::Vec3d;
use rust_3d_course::object::{Cube, Object, ObjectNameTag};

fn window_conf() -> Conf {
    Conf {
        window_title: "MTUCI 3D ENGINE".to_string(),
        window_width: 1000,
        window_height: 750,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    next_frame().await;
    let mut color = RED;
    color.a = 0.5;
    let mut cube = Cube::new(ObjectNameTag::new("Cube"), 2.5, color);
    cube.translate(&Vec3d::new(0.0, 0.0, 7.));
    let cube = Rc::new(RefCell::new(cube));
    let mut camera = MyCamera::new("Camera");
    camera.init(screen_width() as i32, screen_height() as i32, 90.0, -10., 500.);

    loop {
        if is_key_down(KeyCode::Q) && is_key_down(KeyCode::LeftControl) {
            break;
        }

        if is_key_down(KeyCode::D) {
            camera.translate(&Vec3d::new(-5. * get_frame_time() as f64, 0.0, 0.0));
        }
        if is_key_down(KeyCode::A) {
            camera.translate(&Vec3d::new(5. * get_frame_time() as f64, 0.0, 0.0));
        }
        if is_key_down(KeyCode::W) {
            camera.translate(&Vec3d::new(0.0, 0.0, 5. * get_frame_time() as f64));
        }
        if is_key_down(KeyCode::S) {
            camera.translate(&Vec3d::new(0.0, 0.0, -5. * get_frame_time() as f64));
        }
        if is_key_down(KeyCode::Space) {
            camera.translate(&Vec3d::new(0.0, 5. * get_frame_time() as f64, 0.0));
        }
        if is_key_down(KeyCode::LeftShift) {
            camera.translate(&Vec3d::new(0.0, -5. * get_frame_time() as f64, 0.0));
        }

        clear_background(LIGHTGRAY);
        // cube.borrow_mut().rotate_left((PI / 8. * get_frame_time()) as f64);
        // cube.borrow_mut().rotate_up((PI / 4. * get_frame_time()) as f64);
        // cube.borrow_mut().rotate_look_at((PI / 2. * get_frame_time()) as f64);
        cube.borrow_mut().rotate(&Vec3d::new(
            0.25 * get_frame_time() as f64,
            0.25 * get_frame_time() as f64 * 0.5,
            0.25 * get_frame_time() as f64 * 0.25,
        ));
        camera.project(cube.clone());
        let tris = camera.sorted();
        // println!("{} tris", tris.len());
        for tri in tris {
            let p = tri.points();
            let p1 = Vec2::new(p[0].x() as f32, p[0].y() as f32);
            let p2 = Vec2::new(p[1].x() as f32, p[1].y() as f32);
            let p3 = Vec2::new(p[2].x() as f32, p[2].y() as f32);
            draw_triangle(p1, p2, p3, tri.color());
            draw_triangle_lines(p1, p2, p3, 1.5, BLACK)
        }
        camera.clear();
        next_frame().await;
    }
}
