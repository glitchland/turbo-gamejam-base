use crate::Component;

#[derive(Component, Clone)]
pub struct Enemy {
    pub color: u32,
    pub health: u32,
    pub speed: u32,
    pub size: u32,
}