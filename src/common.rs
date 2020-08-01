#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mono {
    Zero,
    One,
}

pub trait ActAsMono {
    fn act_as(&self) -> Mono;
}

impl ActAsMono for Mono {
    fn act_as(&self) -> Mono {
        *self
    }
}

impl ActAsMono for u8 {
    fn act_as(&self) -> Mono {
        match self {
            0 => Mono::Zero,
            _ => Mono::One,
        }
    }
}

pub trait ActAsXywh {
    fn xywh(&self) -> (usize, usize, usize, usize);
}

impl ActAsXywh for (usize, usize, usize, usize) {
    fn xywh(&self) -> (usize, usize, usize, usize) {
        *self
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl ActAsXywh for Rectangle {
    fn xywh(&self) -> (usize, usize, usize, usize) {
        let Rectangle {
            x,
            y,
            width,
            height,
        } = self;
        (*x, *y, *width, *height)
    }
}

impl Rectangle {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
