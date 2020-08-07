pub trait EightData {
    fn core(&self) -> &[u8];
    fn core_mut(&mut self) -> &mut [u8];
    fn as_vev(&self) -> &[u8];
    fn len(&self) -> usize;
}
