use raylib::{RaylibHandle, RaylibThread};
use bevy_ecs::resource::Resource;

use std::ops::{Deref, DerefMut};


#[derive(Resource)]
pub struct Rl(
    pub RaylibHandle,
);
impl Deref for Rl {
    type Target = RaylibHandle;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Rl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Thread(
    pub RaylibThread,
);
impl Deref for Thread {
    type Target = RaylibThread;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Thread {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}