use glam::f32::{Vec3, Quat};

struct Quaternion<T>(T, T, T, T);

struct Craft {
    orientation: Quat,
    angular_velocity: Vec3,
    velocity: Vec3,
    position: Vec3,
}


