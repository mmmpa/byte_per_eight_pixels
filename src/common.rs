pub fn compute_eight_length(src: usize) -> usize {
    match src >> 3 {
        m if m == 0 => 1,
        m if src % 8 == 0 => m,
        m => m + 1,
    }
}

pub fn draw(data: &mut [u8], data_i: usize, shifted_xy: usize, color: Mono) {
    match color {
        Mono::One => {
            data[data_i] |= match shifted_xy % 8 {
                0 => 0b_1000_0000,
                1 => 0b_0100_0000,
                2 => 0b_0010_0000,
                3 => 0b_0001_0000,
                4 => 0b_0000_1000,
                5 => 0b_0000_0100,
                6 => 0b_0000_0010,
                7 => 0b_0000_0001,
                _ => 0,
            }
        }
        Mono::Zero => {
            data[data_i] &= match shifted_xy % 8 {
                0 => 0b_0111_1111,
                1 => 0b_1011_1111,
                2 => 0b_1101_1111,
                3 => 0b_1110_1111,
                4 => 0b_1111_0111,
                5 => 0b_1111_1011,
                6 => 0b_1111_1101,
                7 => 0b_1111_1110,
                _ => 0,
            }
        }
    }
}
pub struct AsEight {
    pub start: usize,
    pub length: usize,
}

pub fn into_as_eight(src: usize, length: usize) -> AsEight {
    let start = src >> 3;
    let end = (src + length - 1) >> 3;
    let length = end - start + 1;

    AsEight { start, length }
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
    #[allow(unused_imports)]
    use crate::compute_eight_length;

    #[test]
    fn test_eight_width() {
        assert_eq!(1, compute_eight_length(7));
        assert_eq!(1, compute_eight_length(8));
        assert_eq!(2, compute_eight_length(9));
        assert_eq!(2, compute_eight_length(16));
        assert_eq!(3, compute_eight_length(17));
    }
}
