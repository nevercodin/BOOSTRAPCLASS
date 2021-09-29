
//
//  msgf_afrm.rs
//	Musical Sound Generator Framework
//      Audio Frame Class
//
//  Created by Hasebe Masahiko on 2021/09/24.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct AudioFrame {
    abuf: Vec<f32>,
    pub sample_number: usize,
    index: usize,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl AudioFrame {
    pub fn new(mut sample_number: usize, total_size: usize) -> Self {
        if sample_number > total_size {
            sample_number = total_size;
        }
        Self {
            abuf: vec![0.0; total_size],
            sample_number,
            index: 0,
        }
    }
    pub fn set_sample_number(&mut self, snum: usize) {
        self.sample_number = snum;      
    }
    pub fn copy_to_sysbuf(&self, ab: &mut [f32; msgf_if::MAX_BUFFER_SIZE]) {
        for i in 0..self.sample_number {
            ab[i] = self.abuf[i];
        }
    }
    pub fn add_to_sysbuf(&self, ab: &mut [f32; msgf_if::MAX_BUFFER_SIZE]) {
        for i in 0..self.sample_number {
            ab[i] += self.abuf[i];
        }
    }
    pub fn _copy_to_abuf(&self, ab: &mut AudioFrame) {
        for i in 0..self.sample_number {
            ab.abuf[i] = self.abuf[i];
        }
    }
    pub fn put_into_abuf(&mut self, val: f32) {
        self.abuf[self.index] = val;
        self.index += 1;
        if self.index >= self.sample_number {