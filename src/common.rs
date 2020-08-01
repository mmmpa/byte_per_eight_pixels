pub fn compute_eight_length(src: usize) -> usize {
    match src >> 3 {
        m if m == 0 => 1,
        m if src % 8 == 0 => m,
        m => m + 1,
    }
}

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

mod test {
    use crate::*;

    #[test]
    fn test_eight_width() {
        assert_eq!(1, compute_eight_length(7));
        assert_eq!(1, compute_eight_length(8));
        assert_eq!(2, compute_eight_length(9));
        assert_eq!(2, compute_eight_length(16));
        assert_eq!(3, compute_eight_length(17));
    }
}
