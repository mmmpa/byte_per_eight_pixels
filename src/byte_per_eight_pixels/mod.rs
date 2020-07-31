use crate::*;
use std::cmp::{max, min};
use std::io::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BytePerEightPixels {
    width: usize,
    height: usize,
    eight_width: usize,
    eight_data: Vec<u8>,
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

impl From<(usize, usize, usize, usize)> for Rectangle {
    fn from((x, y, width, height): (usize, usize, usize, usize)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
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

impl BytePerEightPixels {
    fn compute_eight_width(width: usize) -> usize {
        match width >> 3 {
            m if m == 0 => 1,
            m if width % 8 == 0 => m,
            m => m + 1,
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let eight_width = Self::compute_eight_width(width);

        Self {
            width,
            height,
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
        o.update((0, 0, width, height), src).unwrap();
        Ok(o)
    }

    pub fn with_eight_data(
        width: usize,
        height: usize,
        eight_data: Vec<u8>,
    ) -> BytePerEightPixelsResult<Self> {
        let eight_width = Self::compute_eight_width(width);

        if eight_width * height != eight_data.len() {
            return Err(BytePerEightPixelsError::InvalidLengthData);
        }

        let o = Self {
            width,
            height,
            eight_width,
            eight_data,
        };
        Ok(o)
    }

    pub fn update(
        &mut self,
        xywh: impl ActAsXywh,
        src: &[impl ActAsMono],
    ) -> BytePerEightPixelsResult<()> {
        let (x, y, width, height) = xywh.xywh();

        // avoid unsigned subtract overflow
        if x > self.width || y > self.height {
            return Ok(());
        }

        let data_width = self.eight_width;

        for step_y in 0..min(height, self.height - y) {
            for step_x in 0..min(width, self.width - x) {
                let color = src[width * step_y + step_x].act_as();
                let data_x = x + step_x;
                let data_y = y + step_y;
                let data_i = data_width * data_y + (data_x >> 3);

                match color {
                    Mono::One => {
                        self.eight_data[data_i] |= match data_x % 8 {
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
                        self.eight_data[data_i] &= match data_x % 8 {
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

    pub fn part_vec(&self, xywh: impl ActAsXywh) -> (Rectangle, Vec<u8>) {
        let (x, y, width, height) = xywh.xywh();
        let src = &self.eight_data;
        let src_width = self.eight_width;

        let AsEight {
            x: src_x,
            width: result_width,
        } = into_as_eight(x, width);

        let mut result = vec![0u8; (result_width * height)];

        for step_y in 0..height {
            for step_x in 0..result_width {
                let real_i = src_width * (y + step_y) + src_x + step_x;
                let result_i = result_width * step_y + step_x;

                result[result_i] = src[real_i];
            }
        }

        (
            Rectangle::new(src_x * 8, y, result_width * 8, height),
            result,
        )
    }
}

struct AsEight {
    pub x: usize,
    pub width: usize,
}

fn into_as_eight(x: usize, width: usize) -> AsEight {
    let x_start = x >> 3;
    let x_end = (x + width - 1) >> 3;
    let byte_width = x_end - x_start + 1;

    AsEight {
        x: x_start,
        width: byte_width,
    }
}

#[cfg(test)]
mod test {
    use crate::*;

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
            .update(
                (6, 1, 3, 2),
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

        let (n, re) = image.part_vec((6, 1, 3, 2));
        assert_eq!(
            #[rustfmt::skip]
            vec![
                0b_0000_0001, 0b_0000_0000,
                0b_0000_0110, 0b_1000_0000,
            ],
            re
        );

        let (n, re) = image.part_vec((0, 0, 3, 1));
        assert_eq!(vec![0b_0000_0000,], re);
        assert_eq!((0, 0, 8, 1), n.xywh());

        let (n, re) = image.part_vec((0, 1, 3, 1));
        assert_eq!(vec![0b_0000_0001,], re);
        assert_eq!((0, 1, 8, 1), n.xywh());

        let (n, re) = image.part_vec((7, 2, 1, 1));
        assert_eq!(vec![0b_0000_0110,], re);
        assert_eq!((0, 2, 8, 1), n.xywh());

        let (n, re) = image.part_vec((8, 2, 1, 1));
        assert_eq!(vec![0b_1000_0000,], re);
        assert_eq!((8, 2, 8, 1), n.xywh());

        let (n, re) = image.part_vec((7, 2, 2, 1));
        assert_eq!(vec![0b_0000_0110, 0b_1000_0000], re);
        assert_eq!((0, 2, 16, 1), n.xywh());
    }

    #[test]
    fn test_update_overflow() {
        {
            let mut image = BytePerEightPixels::new(8, 4);

            image
                .update(
                    (6, 2, 3, 2),
                    #[rustfmt::skip]
                    &vec![
                        1, 1, 1,
                        1, 1, 1,
                        1, 1, 1,
                    ],
                )
                .unwrap();

            assert_eq!(
                #[rustfmt::skip]
                [
                    0b_0000_0000,
                    0b_0000_0000,
                    0b_0000_0011,
                    0b_0000_0011,
                ],
                image.as_vec()
            );
        }
        {
            let mut image = BytePerEightPixels::new(8, 4);

            image.update((0, 5, 1, 1), &vec![1]).unwrap();

            assert_eq!(
                #[rustfmt::skip]
                [
                    0b_0000_0000,
                    0b_0000_0000,
                    0b_0000_0000,
                    0b_0000_0000,
                ],
                image.as_vec()
            );
        }
        {
            let mut image = BytePerEightPixels::new(8, 4);

            image.update((9, 0, 1, 1), &vec![1]).unwrap();

            assert_eq!(
                #[rustfmt::skip]
                [
                    0b_0000_0000,
                    0b_0000_0000,
                    0b_0000_0000,
                    0b_0000_0000,
                ],
                image.as_vec()
            );
        }
    }
}
