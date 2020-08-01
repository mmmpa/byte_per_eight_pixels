use crate::*;
use std::cmp::min;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VerticalEightPxUintEight {
    width: usize,
    height: usize,
    eight_length: usize,
    eight_data: Vec<u8>,
}

impl VerticalEightPxUintEight {
    pub fn new(width: usize, height: usize) -> Self {
        let eight_length = compute_eight_length(height);

        Self {
            width,
            height,
            eight_length,
            eight_data: vec![0; width * eight_length],
        }
    }

    pub fn with_data(
        width: usize,
        height: usize,
        src: &[impl ActAsMono],
    ) -> EightPxUintEightResult<Self> {
        if width * height != src.len() {
            return Err(EightPxUintEightError::InvalidLengthData);
        }

        let mut o = Self::new(width, height);
        o.update((0, 0, width, height), src).unwrap();
        Ok(o)
    }

    pub fn with_eight_data(
        width: usize,
        eight_height: usize,
        eight_data: Vec<u8>,
    ) -> EightPxUintEightResult<Self> {
        if width * eight_height != eight_data.len() {
            return Err(EightPxUintEightError::InvalidLengthData);
        }

        let o = Self {
            width,
            height: eight_height * 8,
            eight_length: eight_height,
            eight_data,
        };
        Ok(o)
    }

    pub fn update(
        &mut self,
        xywh: impl ActAsXywh,
        src: &[impl ActAsMono],
    ) -> EightPxUintEightResult<()> {
        let (x, y, width, height) = xywh.xywh();

        // avoid unsigned subtract overflow
        if x > self.width || y > self.height {
            return Ok(());
        }

        // discard pixels that overflow
        for step_y in 0..min(height, self.height - y) {
            for step_x in 0..min(width, self.width - x) {
                let color = src[width * step_y + step_x].act_as();
                let data_x = x + step_x;
                let data_y = y + step_y;
                let data_i = self.width * (data_y >> 3) + data_x;

                draw(&mut self.eight_data, data_i, data_y, color);
            }
        }

        Ok(())
    }

    pub fn as_vec(&self) -> &[u8] {
        &self.eight_data
    }

    /// Return rectangle as 1 cell has 8 pixels.
    pub fn part_vec(&self, xywh: impl ActAsXywh) -> (Rectangle, Vec<u8>) {
        let (x, y, width, height) = xywh.xywh();

        // avoid unsigned subtract overflow
        if x > self.width || y > self.height {
            return (Rectangle::new(0, 0, 0, 0), vec![]);
        }

        let src = &self.eight_data;
        let src_width = self.width;
        let src_height = self.eight_length;

        let AsEight {
            start: src_y,
            length: result_height,
        } = into_as_eight(y, height);

        let mut result = vec![0u8; width * result_height];

        let result_height = min(result_height, src_height - src_y);
        let result_width = min(width, src_width - x);

        for step_y in 0..result_height {
            for step_x in 0..result_width {
                let real_i = src_width * (src_y + step_y) + x + step_x;
                let result_i = result_width * step_y + step_x;

                result[result_i] = src[real_i];
            }
        }

        (
            Rectangle::new(x, src_y, result_width, result_height),
            result,
        )
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test {
    use crate::*;

    #[test]
    fn test() {
        let data = vec![
            1, 0, 1, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let image = VerticalEightPxUintEight::with_data(8, 3, &data).unwrap();

        assert_eq!(
            [
                0b_1100_0000,
                0b_0000_0000,
                0b_1000_0000,
                0b_0000_0000,
                0b_0100_0000,
                0b_0000_0000,
                0b_1000_0000,
                0b_0100_0000,
            ],
            image.as_vec()
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_invalid_meta() {
        let data = vec![
            1, 1, 0, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let image = VerticalEightPxUintEight::with_data(8, 2, &data);

        assert!(image.is_err());
    }

    #[test]
    fn test_short() {
        let data = vec![
            1, 1, 0, 0, 0,
            1, 0, 0, 0, 1,
            0, 0, 0, 0, 0,
        ];

        let image = VerticalEightPxUintEight::with_data(5, 3, &data).unwrap();

        assert_eq!(
            #[rustfmt::skip]
            [
                0b_1100_0000,
                0b_1000_0000,
                0b_0000_0000,
                0b_0000_0000,
                0b_0100_0000,
            ],
            image.as_vec()
        );
    }

    #[test]
    fn test_long() {
        let data = vec![
            1, 1, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0,

            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 1, 0, 0, 0, 0,
        ];

        let image = VerticalEightPxUintEight::with_data(8, 10, &data).unwrap();

        assert_eq!(
            [
                0b_1000_0000,
                0b_1000_0000,
                0b_0000_0000,
                0b_0000_0001,
                0b_0000_0000,
                0b_0000_0000,
                0b_1000_0000,
                0b_0000_0000,

                0b_1000_0000,
                0b_0000_0000,
                0b_0000_0000,
                0b_0100_0000,
                0b_1000_0000,
                0b_0000_0000,
                0b_0000_0000,
                0b_1000_0000,
            ],
            image.as_vec()
        );
    }

    #[test]
    fn test_update() {
        let data = vec![
            1, 1, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0,

            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 1, 0, 0, 0, 0,
        ];

        let mut image = VerticalEightPxUintEight::with_data(8, 10, &data).unwrap();

        image
            .update(
                (2, 7, 2, 3),
                &vec![
                    Mono::One,
                    Mono::Zero,
                    Mono::Zero,
                    Mono::One,
                    Mono::One,
                    Mono::Zero,
                ],
            )
            .unwrap();

        assert_eq!(
            [
                0b_1000_0000,
                0b_1000_0000,
                0b_0000_0001,
                0b_0000_0000,
                0b_0000_0000,
                0b_0000_0000,
                0b_1000_0000,
                0b_0000_0000,

                0b_1000_0000,
                0b_0000_0000,
                0b_0100_0000,
                0b_1000_0000,
                0b_1000_0000,
                0b_0000_0000,
                0b_0000_0000,
                0b_1000_0000,
            ],
            image.as_vec()
        );

        let (n, re) = image.part_vec((2, 7, 2, 3));
        assert_eq!(
            #[rustfmt::skip]
            vec![
                0b_0000_0001,
                0b_0000_0000,

                0b_0100_0000,
                0b_1000_0000,
            ],
            re
        );
        assert_eq!((2, 0, 2, 2), n.xywh());

        let (n, re) = image.part_vec((0, 0, 1, 3));
        assert_eq!(vec![0b_1000_0000,], re);
        assert_eq!((0, 0, 1, 1), n.xywh());

        let (n, re) = image.part_vec((2, 1, 1, 3));
        assert_eq!(vec![0b_0000_0001,], re);
        assert_eq!((2, 0, 1, 1), n.xywh());

        let (n, re) = image.part_vec((2, 7, 1, 1));
        assert_eq!(vec![0b_0000_0001,], re);
        assert_eq!((2, 0, 1, 1), n.xywh());

        let (n, re) = image.part_vec((2, 8, 1, 1));
        assert_eq!(vec![0b_0100_0000,], re);
        assert_eq!((2, 1, 1, 1), n.xywh());

        let (n, re) = image.part_vec((2, 7, 1, 2));
        assert_eq!(vec![0b_0000_0001, 0b_0100_0000], re);
        assert_eq!((2, 0, 1, 2), n.xywh());

        let (n, re) = image.part_vec((0, 13, 1, 2));
        assert_eq!(0, re.len());
        assert_eq!((0, 0, 0, 0), n.xywh());

        let (n, re) = image.part_vec((0, 13, 2, 1));
        assert_eq!(0, re.len());
        assert_eq!((0, 0, 0, 0), n.xywh());
    }

    #[test]
    fn test_update_overflow() {
        {
            let mut image = VerticalEightPxUintEight::new(4, 8);

            image
                .update(
                    (2, 6, 3, 2),
                    &vec![
                        1, 1, 1,
                        1, 1, 1,
                        1, 1, 1,
                    ],
                )
                .unwrap();

            assert_eq!(
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
            let mut image = VerticalEightPxUintEight::new(4, 8);

            image.update((5, 0, 1, 1), &vec![1]).unwrap();

            assert_eq!(
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
            let mut image = VerticalEightPxUintEight::new(4, 8);

            image.update((0, 9, 1, 1), &vec![1]).unwrap();

            assert_eq!(
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
