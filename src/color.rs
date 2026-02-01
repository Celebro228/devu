use bevy_ecs::component::Component;


#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Component)]
pub struct Color{
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    #[inline]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a, }
    }
}


#[inline]
pub const fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::new(
        r as f32 / 255.,
        g as f32 / 255.,
        b as f32 / 255.,
        a as f32 / 255.,
    )
}


pub const LIGHTGRAY:  Color = color(200, 200, 200, 255);
pub const GRAY:       Color = color(130, 130, 130, 255);
pub const DARKGRAY:   Color = color( 80,  80,  80, 255);
pub const YELLOW:     Color = color(253, 249,   0, 255);
pub const GOLD:       Color = color(255, 203,   0, 255);
pub const ORANGE:     Color = color(255, 161,   0, 255);
pub const PINK:       Color = color(255, 109, 194, 255);
pub const RED:        Color = color(230,  41,  55, 255);
pub const MAROON:     Color = color(190,  33,  55, 255);
pub const GREEN:      Color = color(  0, 228,  48, 255);
pub const LIME:       Color = color(  0, 158,  47, 255);
pub const DARKGREEN:  Color = color(  0, 117,  44, 255);
pub const SKYBLUE:    Color = color(102, 191, 255, 255);
pub const BLUE:       Color = color(  0, 121, 241, 255);
pub const DARKBLUE:   Color = color(  0,  82, 172, 255);
pub const PURPLE:     Color = color(200, 122, 255, 255);
pub const VIOLET:     Color = color(135,  60, 190, 255);
pub const DARKPURPLE: Color = color(112,  31, 126, 255);
pub const BEIGE:      Color = color(211, 176, 131, 255);
pub const BROWN:      Color = color(127, 106,  79, 255);
pub const DARKBROWN:  Color = color( 76,  63,  47, 255);
pub const WHITE:      Color = color(255, 255, 255, 255);
pub const BLACK:      Color = color(  0,   0,   0, 255);
pub const BLANK:      Color = color(  0,   0,   0,   0);
pub const MAGENTA:    Color = color(255,   0, 255, 255);
pub const RAYWHITE:   Color = color(245, 245, 245, 255);