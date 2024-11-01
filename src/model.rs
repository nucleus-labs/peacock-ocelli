
use glam::*;

pub struct Mesh {
    vertices: Vec<Vec3>,
    indices: Option<Vec<u32>>,
    normals: Vec<Vec3>,
    uvs: Option<Vec<Vec2>>,
}

pub struct Material {
    texture: (), // todo
    // more?
}

pub struct Model {
    transform: crate::Transform,
    mesh: Mesh,
    mat: Material,
}
