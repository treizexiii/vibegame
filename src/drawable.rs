use raylib::prelude::RaylibDrawHandle;

// Définir un trait pour les éléments qui peuvent être dessinés
pub trait Drawable {
    fn draw(&self, d: &mut RaylibDrawHandle, screen_x: f32, screen_width: f32);
    fn get_position(&self) -> (f32, f32);
    fn get_size(&self) -> (f32, f32);
    fn get_layer(&self) -> i32;
}