use crate::*;
use std::cmp::min;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VerticalEightPxUintEight<D: EightData> {
    width: usize,
    height: usize,
    eight_length: usize,
    eight_data: D,
}

impl<D: EightData> VerticalEightPxUintEight<D> {
    pub fn new(width: usize, height: usize, eight_data: D) -> EightPxUintEightResult<Self> {
        let eight_length = compute_eight_length(height);

        if width * eight_length != eight_data.len() {
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

        // discard pixels that overflow
        for step_y in 0..min(height, self.height - y) {
            for step_x in 0..min(width, self.width - x) {
                let color = src[width * step_y + step_x].act_as();
                let data_x = x + step_x;
                let data_y = y + step_y;
                let data_i = self.width * (data_y >> 3) + data_x;

                draw_inverse(&mut self.eight_data.core_mut(), data_i, data_y, color);
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
        let src_width = self.width;
        let src_height = self.eight_length;

        let AsEight {
            start: src_y,
            length: result_height,
        } = into_as_eight(y, height);

        let result_height = min(result_height, src_height - src_y);
        let result_width = min(width, src_width - x);

        for step_y in 0..result_height {
            for step_x in 0..result_width {
                let real_i = src_width * (src_y + step_y) + x + step_x;
                let result_i = result_width * step_y + step_x;

                result[result_i] = src[real_i];
            }
        }

        Rectangle::new(x, src_y, result_width, result_height)
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test {
    use crate::*;


    #[test]
    fn test_vertical() {
        let data = EightDataClient::new(8);
        let image_src = vec![
            1, 0, 1, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let mut image = VerticalEightPxUintEight::new(8, 3, data).unwrap();
        image.update((0, 0, 8, 3), &image_src);

        assert_eq!(
            [
                0b_0000_0011,
                0b_0000_0000,
                0b_0000_0001,
                0b_0000_0000,
                0b_0000_0010,
                0b_0000_0000,
                0b_0000_0001,
                0b_0000_0010,
            ],
            image.as_vec()
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_invalid_meta() {
        let data = EightDataClient::new(9);
        let mut image = VerticalEightPxUintEight::new(8, 2, data);

        assert!(image.is_err());
    }

    #[test]
    fn test_short() {
        let data = EightDataClient::new(5);
        let image_src = vec![
            1, 1, 0, 0, 0,
            1, 0, 0, 0, 1,
            0, 0, 0, 0, 0,
        ];

        let mut image = VerticalEightPxUintEight::new(5, 3, data).unwrap();
        image.update((0, 0, 5, 3), &image_src);

        assert_eq!(
            [
                0b_0000_0011,
                0b_0000_0001,
                0b_0000_0000,
                0b_0000_0000,
                0b_0000_0010,
            ],
            image.as_vec()
        );
    }

    #[test]
    fn test_long() {
        let data = EightDataClient::new(16);
        let image_src = vec![
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

        let mut image = VerticalEightPxUintEight::new(8, 10, data).unwrap();
        image.update((0, 0, 8, 10), &image_src);

        assert_eq!(
            [
                0b_0000_0001,
                0b_0000_0001,
                0b_0000_0000,
                0b_1000_0000,
                0b_0000_0000,
                0b_0000_0000,
                0b_0000_0001,
                0b_0000_0000,

                0b_0000_0001,
                0b_0000_0000,
                0b_0000_0000,
                0b_0000_0010,
                0b_0000_0001,
                0b_0000_0000,
                0b_0000_0000,
                0b_0000_0001,
            ],
            image.as_vec()
        );
    }

    #[test]
    fn test_update() {
        let data = EightDataClient::new(16);
        let image_src = vec![
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

        let mut image = VerticalEightPxUintEight::new(8, 10, data).unwrap();
        image.update((0, 0, 8, 10), &image_src);

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
                0b_0000_0001,
                0b_0000_0001,
                0b_1000_0000,
                0b_0000_0000,
                0b_0000_0000,
                0b_0000_0000,
                0b_0000_0001,
                0b_0000_0000,

                0b_0000_0001,
                0b_0000_0000,
                0b_0000_0010,
                0b_0000_0001,
                0b_0000_0001,
                0b_0000_0000,
                0b_0000_0000,
                0b_0000_0001,
            ],
            image.as_vec()
        );

        let mut re = [0;4];

        let n = image.part_vec((2, 7, 2, 3), &mut re);
        assert_eq!(
            vec![
                0b_1000_0000,
                0b_0000_0000,

                0b_0000_0010,
                0b_0000_0001,
            ],
            re
        );
        assert_eq!((2, 0, 2, 2), n.xywh());

        let n = image.part_vec((0, 0, 1, 3), &mut re);
        assert_eq!([0b_0000_0001,], re[0..1]);
        assert_eq!((0, 0, 1, 1), n.xywh());

        let n = image.part_vec((2, 1, 1, 3), &mut re);
        assert_eq!([0b_1000_0000,], re[0..1]);
        assert_eq!((2, 0, 1, 1), n.xywh());

        let n = image.part_vec((2, 7, 1, 1), &mut re);
        assert_eq!([0b_1000_0000,], re[0..1]);
        assert_eq!((2, 0, 1, 1), n.xywh());

        let n = image.part_vec((2, 8, 1, 1), &mut re);
        assert_eq!([0b_0000_0010,], re[0..1]);
        assert_eq!((2, 1, 1, 1), n.xywh());

        let n = image.part_vec((2, 7, 1, 2), &mut re);
        assert_eq!([0b_1000_0000, 0b_0000_0010], re[0..2]);
        assert_eq!((2, 0, 1, 2), n.xywh());

        let n = image.part_vec((0, 13, 1, 2), &mut re);
        assert_eq!((0, 0, 0, 0), n.xywh());

        let n = image.part_vec((0, 13, 2, 1), &mut re);
        assert_eq!((0, 0, 0, 0), n.xywh());
    }

    #[test]
    fn test_update_overflow() {
        {
            let mut image = VerticalEightPxUintEight::new(4, 8, EightDataClient::new(4)).unwrap();

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
                    0b_1100_0000,
                    0b_1100_0000,
                ],
                image.as_vec()
            );
        }
        {
            let mut image = VerticalEightPxUintEight::new(4, 8, EightDataClient::new(4)).unwrap();

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
            let mut image = VerticalEightPxUintEight::new(4, 8, EightDataClient::new(4)).unwrap();

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
