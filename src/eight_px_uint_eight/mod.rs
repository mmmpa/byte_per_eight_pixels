use crate::*;
use std::cmp::min;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EightPxUintEight<D: EightData> {
    width: usize,
    height: usize,
    eight_length: usize,
    eight_data: D,
}

impl<D: EightData> EightPxUintEight<D> {
    pub fn new(width: usize, height: usize, eight_data: D) -> EightPxUintEightResult<Self> {
        let eight_length = compute_eight_length(width);

        if eight_length * height != eight_data.len() {
            return Err(EightPxUintEightError::InvalidLengthData);
        }

        Ok(Self {
            width,
            height,
            eight_length,
            eight_data,
        })
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

        let data_width = self.eight_length;

        // discard pixels that overflow
        for step_y in 0..min(height, self.height - y) {
            for step_x in 0..min(width, self.width - x) {
                let color = src[width * step_y + step_x].act_as();
                let data_x = x + step_x;
                let data_y = y + step_y;
                let data_i = data_width * data_y + (data_x >> 3);

                draw(&mut self.eight_data.core_mut(), data_i, data_x, color);
            }
        }

        Ok(())
    }

    pub fn as_vec(&self) -> &[u8] {
        &self.eight_data.as_vev()
    }

    /// Return rectangle as 1 cell has 8 pixels.
    pub fn part_vec(&self, xywh: impl ActAsXywh, result: &mut [u8]) -> Rectangle {
        let (x, y, width, height) = xywh.xywh();

        // avoid unsigned subtract overflow
        if x > self.width || y > self.height {
            return Rectangle::new(0, 0, 0, 0);
        }

        let src = &self.eight_data.core();
        let src_width = self.eight_length;
        let src_height = self.height;

        let AsEight {
            start: src_x,
            length: result_width,
        } = into_as_eight(x, width);

        let result_height = min(height, src_height - y);
        let result_width = min(result_width, src_width - src_x);

        for step_y in 0..result_height {
            for step_x in 0..result_width {
                let real_i = src_width * (y + step_y) + src_x + step_x;
                let result_i = result_width * step_y + step_x;

                result[result_i] = src[real_i];
            }
        }

        Rectangle::new(src_x, y, result_width, result_height)
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test {
    use crate::*;

    #[test]
    fn test_horizontal() {
        let data = EightDataClient::new(3);
        let image_src = vec![
            1, 1, 0, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let mut image = EightPxUintEight::new(8, 3, data).unwrap();
        image.update((0, 0, 8, 3), &image_src);

        assert_eq!(
            [
                0b_1100_0010,
                0b_1000_1001,
                0b_0000_0000,
            ],
            image.as_vec()
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_invalid_meta() {
        let data = EightDataClient::new(3);

        let mut image = EightPxUintEight::new(8, 2, data);

        assert!(image.is_err());
    }

    #[test]
    fn test_short() {
        let data = EightDataClient::new(3);
        let image_src = vec![
            1, 1, 0, 0, 0,
            1, 0, 0, 0, 1,
            0, 0, 0, 0, 0,
        ];

        let mut image = EightPxUintEight::new(5, 3, data).unwrap();
        image.update((0, 0, 5, 3), &image_src);

        assert_eq!(
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
        let data = EightDataClient::new(6);
        let image_src = vec![
            1, 1, 0, 0, 0, 0, 1, 0,  0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,  1, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,  0, 0, 1,
        ];

        let mut image = EightPxUintEight::new(11, 3, data).unwrap();
        image.update((0, 0, 11, 3), &image_src);

        assert_eq!(
            [
                0b_1100_0010, 0b_0100_0000,
                0b_1000_1001, 0b_1010_0000,
                0b_0000_0000, 0b_0010_0000,
            ],
            image.as_vec()
        );
    }

    #[test]
    fn test_update() {
        let data = EightDataClient::new(6);
        let image_src = vec![
            0, 0, 0, 0, 0, 0, 0, 0,  0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1,  0, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 1,  0, 0, 0,
        ];

        let mut image = EightPxUintEight::new(11, 3, data).unwrap();
        image.update((0, 0, 11, 3), &image_src);

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
            [
                0b_0000_0000, 0b_0000_0000,
                0b_0000_0001, 0b_0000_0000,
                0b_0000_0110, 0b_1000_0000,
            ],
            image.as_vec()
        );

        let mut re = [0; 8];

        let n = image.part_vec((6, 1, 3, 2), &mut re);
        assert_eq!(
            [
                0b_0000_0001, 0b_0000_0000,
                0b_0000_0110, 0b_1000_0000,
            ],
            re[0..4]
        );
        assert_eq!((0, 1, 2, 2), n.xywh());

        let n = image.part_vec((0, 0, 3, 1), &mut re);
        assert_eq!([0b_0000_0000,], re[0..1]);
        assert_eq!((0, 0, 1, 1), n.xywh());

        let n = image.part_vec((0, 1, 3, 1), &mut re);
        assert_eq!([0b_0000_0001,], re[0..1]);
        assert_eq!((0, 1, 1, 1), n.xywh());

        let n = image.part_vec((7, 2, 1, 1), &mut re);
        assert_eq!([0b_0000_0110,], re[0..1]);
        assert_eq!((0, 2, 1, 1), n.xywh());

        let n = image.part_vec((8, 2, 1, 1), &mut re);
        assert_eq!([0b_1000_0000,], re[0..1]);
        assert_eq!((1, 2, 1, 1), n.xywh());

        let n = image.part_vec((7, 2, 2, 1), &mut re);
        assert_eq!([0b_0000_0110, 0b_1000_0000], re[0..2]);
        assert_eq!((0, 2, 2, 1), n.xywh());

        let n = image.part_vec((0, 4, 2, 1), &mut re);
        assert_eq!((0, 0, 0, 0), n.xywh());

        let n = image.part_vec((12, 0, 2, 1), &mut re);

        assert_eq!((0, 0, 0, 0), n.xywh());
    }

    #[test]
    fn test_update_overflow() {
        {
            let data = EightDataClient::new(4);
            let mut image = EightPxUintEight::new(8, 4, data).unwrap();

            image
                .update(
                    (6, 2, 3, 2),
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
            let data = EightDataClient::new(4);
            let mut image = EightPxUintEight::new(8, 4, data).unwrap();

            image.update((0, 5, 1, 1), &vec![1]).unwrap();

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
            let data = EightDataClient::new(4);
            let mut image = EightPxUintEight::new(8, 4, data).unwrap();

            image.update((9, 0, 1, 1), &vec![1]).unwrap();

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
