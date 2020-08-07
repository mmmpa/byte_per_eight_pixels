use crate::*;
use core::cmp::min;

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
}

impl<D: EightData> EightPxUintEight for VerticalEightPxUintEight<D> {
    type EightData = D;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn eight_length(&self) -> usize {
        self.eight_length
    }

    fn eight_data(&self) -> &Self::EightData {
        &self.eight_data
    }

    fn eight_data_mut(&mut self) -> &mut Self::EightData {
        &mut self.eight_data
    }

    fn draw(&mut self, x: usize, y: usize, color: Mono) {
        let data = self.eight_data.core_mut();
        let data_i = self.width * (y >> 3) + x;

        match color {
            Mono::One => {
                data[data_i] |= match y % 8 {
                    0 => 0b_0000_0001,
                    1 => 0b_0000_0010,
                    2 => 0b_0000_0100,
                    3 => 0b_0000_1000,
                    4 => 0b_0001_0000,
                    5 => 0b_0010_0000,
                    6 => 0b_0100_0000,
                    7 => 0b_1000_0000,
                    _ => 0,
                }
            }
            Mono::Zero => {
                data[data_i] &= match y % 8 {
                    0 => 0b_1111_1110,
                    1 => 0b_1111_1101,
                    2 => 0b_1111_1011,
                    3 => 0b_1111_0111,
                    4 => 0b_1110_1111,
                    5 => 0b_1101_1111,
                    6 => 0b_1011_1111,
                    7 => 0b_0111_1111,
                    _ => 0,
                }
            }
        }
    }

    fn compute_part(&self, xywh: impl ActAsXywh) -> Part {
        let (x, y, width, height) = xywh.xywh();

        let src_width = self.width;
        let src_height = self.eight_length;

        let AsEight {
            start: src_y,
            length: result_height,
        } = into_as_eight(y, height);

        let src_x = x;
        let result_height = min(result_height, src_height - src_y);
        let result_width = min(width, src_width - x);

        Part::new(
            src_x,
            src_y,
            src_width,
            src_height,
            result_width,
            result_height,
        )
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
#[rustfmt::skip]
mod test {
    use crate::*;
    use crate::unix::EightDataClient;

    #[test]
    fn test_vertical() {
        let data = EightDataClient::new(8);
        let image_src = vec![
            1, 0, 1, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let mut image = VerticalEightPxUintEight::new(8, 3, data).unwrap();
        image.update((0, 0, 8, 3), &image_src).unwrap();

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
        let image = VerticalEightPxUintEight::new(8, 2, data);

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
        image.update((0, 0, 5, 3), &image_src).unwrap();

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
        image.update((0, 0, 8, 10), &image_src).unwrap();

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
        image.update((0, 0, 8, 10), &image_src).unwrap();

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
