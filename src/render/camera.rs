use core::{f32::consts::FRAC_PI_2, time::Duration};

use cgmath::{Matrix4, Rad, Vector3};
use glium::glutin::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseScrollDelta, VirtualKeyCode},
};

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

pub struct Camera {
    pub position: nalgebra::Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new<V: Into<nalgebra::Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        let y = nalgebra::Vector3::y();
        let target =
            nalgebra::Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize();

        let ret = Matrix4::look_to_rh(
            cgmath::Point3::new(self.position.x, self.position.y, self.position.z),
            Vector3::new(target.x, target.y, target.z),
            Vector3::new(y.x, y.y, y.z),
        );

        let a =
            nalgebra::Isometry3::look_at_rh(&self.position, &target.into(), &y).to_homogeneous();

        #[rustfmt::skip]
        let ret2 = Matrix4::new(
            a.m11, a.m21, a.m31, a.m41,
            a.m12, a.m22, a.m32, a.m42,
            a.m13, a.m23, a.m33, a.m43,
            a.m14, a.m24, a.m34, a.m44,
        );
        ret
    }
}

#[derive(Debug)]
pub struct Projection {
    aspect: f32,
    fov_y: Rad<f32>,
    z_near: f32,
    z_far: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(
        width: u32,
        height: u32,
        fov_y: F,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fov_y: fov_y.into(),
            z_near,
            z_far,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}

#[derive(Debug)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backwards: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.,
            amount_right: 0.,
            amount_forward: 0.,
            amount_backwards: 0.,
            amount_up: 0.,
            amount_down: 0.,
            rotate_horizontal: 0.,
            rotate_vertical: 0.,
            scroll: 0.,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let amount = (state == ElementState::Pressed) as u32 as f32;

        let mut res = true;

        match key {
            VirtualKeyCode::Z | VirtualKeyCode::W => self.amount_forward = amount,
            VirtualKeyCode::S => self.amount_backwards = amount,
            VirtualKeyCode::Q | VirtualKeyCode::A => self.amount_left = amount,
            VirtualKeyCode::D => self.amount_right = amount,
            VirtualKeyCode::Space => self.amount_up = amount,
            VirtualKeyCode::LShift => self.amount_down = amount,
            _ => res = false,
        }

        res
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration) {
        let dt = dt.as_secs_f32();

        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = nalgebra::Vector3::new(yaw_cos, 0., yaw_sin).normalize();
        let right = nalgebra::Vector3::new(-yaw_sin, 0., yaw_cos).normalize();

        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward =
            nalgebra::Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();

        let spd = self.speed * dt;
        let sens = self.sensitivity * dt;

        camera.position += scrollward * (self.amount_forward - self.amount_backwards) * spd;
        camera.position += right * (self.amount_right - self.amount_left) * spd;

        camera.position += scrollward * self.scroll * self.speed * sens;

        camera.position.y += (self.amount_up - self.amount_down) * spd;

        camera.yaw += Rad(self.rotate_horizontal) * sens;
        camera.pitch += Rad(-self.rotate_vertical) * sens;

        self.scroll = 0.;
        self.rotate_horizontal = 0.;
        self.rotate_vertical = 0.;

        camera.pitch = Rad(camera.pitch.0.clamp(-SAFE_FRAC_PI_2, SAFE_FRAC_PI_2));
    }
}
