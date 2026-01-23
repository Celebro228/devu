use bevy_ecs::component::Component;
use num_traits::AsPrimitive;
use glam::Vec2;


#[derive(Component)]
pub enum Shape {
    Circle(f32),
    Rect(f32, f32),
    Line(Vec2, Vec2, f32),
}


#[inline(always)]
pub fn circle<R: AsPrimitive<f32>>(r: R) -> Shape {
    Shape::Circle(r.as_())
}

#[inline(always)]
pub fn rect<W, H>(w: W, h: H) -> Shape
where
    W: AsPrimitive<f32>,
    H: AsPrimitive<f32>
{
    Shape::Rect(w.as_(), h.as_())
}

#[inline(always)]
pub fn line<T: AsPrimitive<f32>>(start_pos: Vec2, end_pos: Vec2, thick: T) -> Shape {
    Shape::Line(start_pos, end_pos, thick.as_())
}