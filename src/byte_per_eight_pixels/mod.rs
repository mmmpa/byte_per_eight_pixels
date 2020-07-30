use crate::*;
use std::cmp::max;
use std::io::Write;

pub struct BytePerEightPixels {
    width: usize,
    height: usize,
    eight_width: usize,
    data: Vec<Mono>,
    eight_data: Vec<u8>,
}

#[derive(Eq, PartialEq, Copy, Clone)]
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

impl BytePerEightPixels {
    pub fn new(width: usize, height: usize) -> Self {
        let eight_width = match width >> 3 {
            m if m == 0 => 1,
            m if width % 8 == 0 => m,
            m => m + 1,
        };

        Self {
            width,
            height,
            data: vec![Mono::Zero; width * height],
            eight_width,
            eight_data: vec![0; eight_width * height],
        }
    }

    pub fn with_data(
        width: usize,
        height: usize,
        src: &[impl ActAsMono],
    ) -> BytePerEightPixelsResult<Self> {
        if width * height != src.len() {
            return Err(BytePerEightPixelsError::InvalidLengthData);
        }

        let mut o = Self::new(width, height);
        o.set(0, 0, width, height, src);
        Ok(o)
    }

    pub fn set(
        &mut self,
        start_x: usize,
        start_y: usize,
        w: usize,
        h: usize,
        src: &[impl ActAsMono],
    ) -> BytePerEightPixelsResult<()> {
        for y in 0..h {
            for x in 0..w {
                let color = src[w * y + x].act_as();
                let real_x = start_x + x;
                let real_y = start_y + y;

                let position = (self.eight_width * real_y + (real_x >> 3));

                match color {
                    Mono::One => {
                        self.eight_data[position] |= match real_x % 8 {
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
                        self.eight_data[position] &= match real_x % 8 {
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
        }

        Ok(())
    }

    pub fn as_vec(&self) -> &[u8] {
        &self.eight_data
    }

    pub fn as_part_vec(
        &self,
        x: usize,
        real_byte_start_y: usize,
        width: usize,
        clipped_height: usize,
    ) -> ((usize, usize, usize, usize), Vec<u8>) {
        let real_byte_width = self.eight_width;

        let eight = Eight { x, width };

        let real_byte_start_x = eight.x_start();
        let clipped_byte_width = eight.byte_width();

        let mut clipped = vec![0u8; (clipped_byte_width * clipped_height)];

        for y in 0..clipped_height {
            for x in 0..clipped_byte_width {
                let real_position =
                    real_byte_width * (real_byte_start_y + y) + real_byte_start_x + x;
                let clipped_position = clipped_byte_width * y + x;

                clipped[clipped_position] = self.eight_data[real_position];
            }
        }

        let normalized = (
            real_byte_start_x * 8,
            real_byte_start_y,
            clipped_byte_width * 8,
            clipped_height,
        );
        (normalized, clipped)
    }
}

pub struct Eight {
    pub x: usize,
    pub width: usize,
}

impl Eight {
    pub fn x_start(&self) -> usize {
        self.x >> 3
    }

    fn x_end(&self) -> usize {
        (self.x + self.width - 1) >> 3
    }

    pub fn byte_width(&self) -> usize {
        self.x_end() - self.x_start() + 1
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    fn result(image: &BytePerEightPixels) -> String {
        image
            .as_vec()
            .into_iter()
            .fold("".to_string(), |a, byte| a + &format!("{:>08b}\n", byte))
    }

    #[test]
    fn test() {
        #[rustfmt::skip]
        let data = vec![
            1, 1, 0, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let image = BytePerEightPixels::with_data(8, 3, &data).unwrap();

        #[rustfmt::skip]
        assert_eq!(
            #[rustfmt::skip]
            [
                0b_1100_0010,
                0b_1000_1001,
                0b_0000_0000,
            ],
            image.as_vec()
        );
    }

    #[test]
    fn test_invalid_meta() {
        #[rustfmt::skip]
        let data = vec![
            1, 1, 0, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let image = BytePerEightPixels::with_data(8, 2, &data);

        assert!(image.is_err());
    }

    #[test]
    fn test_short() {
        #[rustfmt::skip]
        let data = vec![
            1, 1, 0, 0, 0,
            1, 0, 0, 0, 1,
            0, 0, 0, 0, 0,
        ];

        let image = BytePerEightPixels::with_data(5, 3, &data).unwrap();

        assert_eq!(
            #[rustfmt::skip]
            [
                0b_1100_0000,
                0b_1000_1000,
                0b_0000_0000,
            ],
            image.as_vec()
        );
    }

    #[test]
    fn test_long() {
        #[rustfmt::skip]
        let data = vec![
            1, 1, 0, 0, 0, 0, 1, 0,  0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,  1, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,  0, 0, 1,
        ];

        let image = BytePerEightPixels::with_data(11, 3, &data).unwrap();

        assert_eq!(
            #[rustfmt::skip]
            [
                0b_1100_0010, 0b_0100_0000,
                0b_1000_1001, 0b_1010_0000,
                0b_0000_0000, 0b_0010_0000,
            ],
            image.as_vec()
        );
    }

    fn test_print(data: &[u8]) {
        data.iter().for_each(|r| println!("{:>08b}", r));
    }

    #[test]
    fn test_update() {
        #[rustfmt::skip]
        let data = vec![
            0, 0, 0, 0, 0, 0, 0, 0,  0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1,  0, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 1,  0, 0, 0,
        ];

        let mut image = BytePerEightPixels::with_data(11, 3, &data).unwrap();

        image
            .set(
                6,
                1,
                3,
                2,
                &vec![
                    Mono::Zero,
                    Mono::One,
                    Mono::Zero,
                    Mono::One,
                    Mono::Zero,
                    Mono::One,
                ],
            )
            .unwrap();

        assert_eq!(
            #[rustfmt::skip]
            [
                0b_0000_0000, 0b_0000_0000,
                0b_0000_0001, 0b_0000_0000,
                0b_0000_0110, 0b_1000_0000,
            ],
            image.as_vec()
        );

        let (n, re) = image.as_part_vec(6, 1, 3, 2);
        assert_eq!(
            #[rustfmt::skip]
            vec![
                0b_0000_0001, 0b_0000_0000,
                0b_0000_0110, 0b_1000_0000,
            ],
            re
        );

        let (n, re) = image.as_part_vec(0, 0, 3, 1);
        assert_eq!(vec![0b_0000_0000,], re);
        assert_eq!((0, 0, 8, 1), n);

        let (n, re) = image.as_part_vec(0, 1, 3, 1);
        assert_eq!(vec![0b_0000_0001,], re);
        assert_eq!((0, 1, 8, 1), n);

        let (n, re) = image.as_part_vec(7, 2, 1, 1);
        assert_eq!(vec![0b_0000_0110,], re);
        assert_eq!((0, 2, 8, 1), n);

        let (n, re) = image.as_part_vec(8, 2, 1, 1);
        assert_eq!(vec![0b_1000_0000,], re);
        assert_eq!((8, 2, 8, 1), n);

        let (n, re) = image.as_part_vec(7, 2, 2, 1);
        assert_eq!(vec![0b_0000_0110, 0b_1000_0000], re);
        assert_eq!((0, 2, 16, 1), n);
    }
}
