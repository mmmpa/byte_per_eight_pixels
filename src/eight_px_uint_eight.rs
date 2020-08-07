use crate::*;
use core::cmp::min;

pub trait EightPxUintEight {
    type EightData: EightData;

    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn eight_length(&self) -> usize;
    fn eight_data(&self) -> &Self::EightData;
    fn eight_data_mut(&mut self) -> &mut Self::EightData;

    fn draw(&mut self, x: usize, y: usize, color: Mono);
    fn compute_part(&self, xywh: impl ActAsXywh) -> Part;

    fn update(
        &mut self,
        xywh: impl ActAsXywh,
        src: &[impl ActAsMono],
    ) -> EightPxUintEightResult<()> {
        let (x, y, width, height) = xywh.xywh();

        // avoid unsigned subtract overflow
        if x > self.width() || y > self.height() {
            return Ok(());
        }

        // discard pixels that overflow
        for step_y in 0..min(height, self.height() - y) {
            for step_x in 0..min(width, self.width() - x) {
                let color = src[width * step_y + step_x].act_as();
                let data_x = x + step_x;
                let data_y = y + step_y;

                self.draw(data_x, data_y, color);
            }
        }

        Ok(())
    }

    fn as_vec(&self) -> &[u8] {
        &self.eight_data().as_vev()
    }

    /// Return rectangle as 1 cell has 8 pixels.
    fn part_vec(&self, xywh: impl ActAsXywh, result: &mut [u8]) -> Rectangle {
        let (x, y, ..) = xywh.xywh();

        // avoid unsigned subtract overflow
        if x > self.width() || y > self.height() {
            return Rectangle::new(0, 0, 0, 0);
        }

        let src = &self.eight_data().core();

        let Part {
            src_x,
            src_y,
            src_width,
            result_width,
            result_height,
            ..
        } = self.compute_part(xywh);

        for step_y in 0..result_height {
            for step_x in 0..result_width {
                let real_i = src_width * (src_y + step_y) + src_x + step_x;
                let result_i = result_width * step_y + step_x;

                result[result_i] = src[real_i];
            }
        }

        Rectangle::new(src_x, src_y, result_width, result_height)
    }
}
