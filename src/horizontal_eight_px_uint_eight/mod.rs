use crate::*;
use core::cmp::min;

pub struct Horizontal;
pub struct Vertical;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct HorizontalEightPxUintEight<D: EightData> {
    width: usize,
    height: usize,
    eight_length: usize,
    eight_data: D,
}

impl<D: EightData> HorizontalEightPxUintEight<D> {
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
}

impl<D: EightData> EightPxUintEight for HorizontalEightPxUintEight<D> {
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
        let data_i = self.eight_length * y + (x >> 3);

        match color {
            Mono::One => {
                data[data_i] |= match x % 8 {
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
                data[data_i] &= match x % 8 {
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

    fn compute_part(&self, xywh: impl ActAsXywh) -> Part {
        let (x, y, width, height) = xywh.xywh();

        let src_width = self.eight_length;
        let src_height = self.height;

        let AsEight {
            start: src_x,
            length: result_width,
        } = into_as_eight(x, width);

        let src_y = y;
        let result_height = min(height, src_height - y);
        let result_width = min(result_width, src_width - src_x);

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
    fn test_horizontal() {
        let data = EightDataClient::new(3);
        let image_src = vec![
            1, 1, 0, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let mut image = HorizontalEightPxUintEight::new(8, 3, data).unwrap();
        image.update((0, 0, 8, 3), &image_src).unwrap();

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

        let image = HorizontalEightPxUintEight::new(8, 2, data);

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

        let mut image = HorizontalEightPxUintEight::new(5, 3, data).unwrap();
        image.update((0, 0, 5, 3), &image_src).unwrap();

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

        let mut image = HorizontalEightPxUintEight::new(11, 3, data).unwrap();
        image.update((0, 0, 11, 3), &image_src).unwrap();

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

        let mut image = HorizontalEightPxUintEight::new(11, 3, data).unwrap();
        image.update((0, 0, 11, 3), &image_src).unwrap();

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
            let mut image = HorizontalEightPxUintEight::new(8, 4, data).unwrap();

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
            let mut image = HorizontalEightPxUintEight::new(8, 4, data).unwrap();

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
            let mut image = HorizontalEightPxUintEight::new(8, 4, data).unwrap();

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
