//
//  msgf_delay.rs
//	Musical Sound Generator Framework
//      Delay Class
//
//  Created by Hasebe Masahiko on 2021/11/27.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;
use crate::engine::msgf_gen::*;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(Copy, Clone)]
pub struct DelayParameter {
    pub l_time: f32,    //  0.0 - 1.0 [sec]
    pub r_time: f32,    //  0.0 - 1.0 [sec]
    pub att_ratio: f32,     //  attenuation
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Delay {
    att_ratio: f32,
    delay_buffer: [msgf_afrm::AudioFrame; 2],
    rd_ptr: [usize;2],
    wr_ptr: [usize;2],
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Delay {

    pub fn new(ref_prms: &DelayParameter) -> Self {
        Delay {
            att_ratio: ref_prms.att_ratio,
            delay_buffer: 
                [msgf_afrm::AudioFrame::new(44100,44100),
                msgf_afrm::AudioFrame::new(44100,44100)],  // 1[sec]
            rd_ptr: [0,0],
            wr_ptr: [(ref_prms.l_time*44100.0) as usize, (ref_prms.r_time*44100.0) as usize],
        }
    }
    fn inc_ptr(&mut self, str: usize) {
        self.rd_ptr[str] += 1;
        self.wr_ptr[str] += 1;
        if self.rd_ptr[str] >= 44100 {
            self.rd_ptr[str] = 0;
        }
        if self.wr_ptr[str] >= 44100 {
            self.wr_ptr[str] = 0;
        }
    }
}
impl Engine for Delay {
    fn process_as(&mut self, in_abuf: [&mut msgf_afrm::AudioFrame;2]) {
        let snum = in_abuf[0].sample_number;
        for str in 0..2 {
            for i in 0..snum {
                if let Some(input_dt) = in_abuf[str].get_from_abuf(i)  {
                    let mut crnt_dt: f32 = 0.0;
                    if let Some(output_dt) = self.delay_buffer[str].get_from_abuf(self.rd_ptr[str]) {
                        crnt_dt = input_dt + output_dt*self.att_ratio;
                        if crnt_dt < msgf_if::DAMP_LIMIT_DEPTH && -msgf_if::DAMP_LIMIT_DEPTH < crnt_dt {
                            crnt_dt = 0.0;
                        }
                        in_abuf