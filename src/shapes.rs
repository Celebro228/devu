use bevy_ecs::component::Component;
use num_traits::AsPrimitive;


#[derive(Component)]
pub enum Shape {
    Circle(f32),
}


#[inline(always)]
pub fn circle<R: AsPrimitive<f32>>(r: R) -> Shape {
    Shape::Circle(r.as_())
}