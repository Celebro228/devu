use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use raylib::texture::Texture2D;
use num_traits::AsPrimitive;
use glam::Vec2;
use hashbrown::HashMap;
use crate::rl::*;


pub(crate) fn init_shape(app: &mut App) {
    app.insert_resource(TextureStore {
        hashmap: HashMap::new(),
    });
    app.add_observer(load_texture);
}


#[derive(Component)]
pub enum Shape {
    Circle(f32),
    Rect(f32, f32),
    Line(Vec2, Vec2, f32),
    Texture(String),
    Text(String, f32),
}

#[derive(Resource)]
pub(crate) struct TextureStore {
    pub hashmap: HashMap<String, Texture2D>,
}


fn load_texture(
    event: On<Add, Shape>,
    mut rl: ResMut<Rl>,
    thread: NonSend<Thread>,
    shapes: Query<&Shape>,
    mut texture_store: ResMut<TextureStore>,
) {
    let shape = shapes.get(event.entity).unwrap();
    if let Shape::Texture(path) = shape {
        if let None = texture_store.hashmap.get(path) {
            match rl.load_texture(&thread, path) {
                Ok(texture) => {
                    texture_store.hashmap.insert(path.clone(), texture);
                }
                Err(error) => {
                    panic!("Error to load texture ({}): {}", path, error);
                }
            }
        }
    }
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

#[inline(always)]
pub fn texture(path: &str) -> Shape {
    Shape::Texture(path.to_string())
}

#[inline(always)]
pub fn text<S: AsPrimitive<f32>>(text: &str, font_size: S) -> Shape {
    Shape::Text(text.to_string(), font_size.as_())
}