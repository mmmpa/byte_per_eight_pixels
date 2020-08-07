#![allow(dead_code)]

use crate::EightData;

pub struct EightDataClient {
    core: Vec<u8>,
}

impl EightDataClient {
    pub fn new(length: usize) -> Self {
        Self {
            core: vec![0; length],
        }
    }
    pub fn with_data(core: Vec<u8>) -> Self {
        Self { core }
    }
}

impl EightData for EightDataClient {
    fn core(&self) -> &[u8] {
        &self.core
    }

    fn core_mut(&mut self) -> &mut [u8] {
        &mut self.core
    }

    fn as_vev(&self) -> &[u8] {
        &self.core
    }

    fn len(&self) -> usize {
        self.core.len()
    }
}
