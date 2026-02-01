pub use glam::{Vec2, Vec3, Vec4};

use num_traits::AsPrimitive;


#[inline(always)]
pub fn vec2<X, Y>(x: X, y: Y) -> Vec2
where
    X: AsPrimitive<f32>,
    Y: AsPrimitive<f32>,
{
    Vec2::new(x.as_(), y.as_())
}

#[inline(always)]
pub fn vec3<X, Y, Z>(x: X, y: Y, z: Z) -> Vec3
where
    X: AsPrimitive<f32>,
    Y: AsPrimitive<f32>,
    Z: AsPrimitive<f32>,
{
    Vec3::new(x.as_(), y.as_(), z.as_())
}

#[inline(always)]
pub fn vec4<X, Y, Z, W>(x: X, y: Y, z: Z, w: W) -> Vec4
where
    X: AsPrimitive<f32>,
    Y: AsPrimitive<f32>,
    Z: AsPrimitive<f32>,
    W: AsPrimitive<f32>,
{
    Vec4::new(x.as_(), y.as_(), z.as_(), w.as_())
}