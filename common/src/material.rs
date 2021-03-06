use color::Color;

#[derive(Clone, DeviceCopy)]
pub enum Material {
    Diffuse { color: Color, albedo: f32 },
    Emissive { emission: Color },
    Reflective,
    Refractive { index: f32 },
}
