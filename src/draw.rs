use std::any::Any;

pub trait Draw {
    fn draw(&mut self);
    fn as_any(&self) -> &dyn Any;
}
