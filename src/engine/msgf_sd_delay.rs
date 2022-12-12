//
//  msgf_sd_delay.rs
//	Musical Sound Generator Framework
//      Send Delay Class
//      ( Send means: 
//          no dry,
//          includes all parts,
//          controled by CC#91 )
//
//  Created by Hasebe Masahiko on 2022/04/11.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::*;
use crate::engine::msgf_delay;
use crate::engine::msgf_gen::*;
//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
//  Same as delay
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
struct SingleBuf {
    delay_buffer: msgf_afrm::AudioFrame,
    rd_ptr: usize,
    wr_ptr: usize,
}
pub struct SdDelay {
    att_ratio: f32,
    dbuf: [SingleBuf; 2],
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl SingleBuf {
    fn init(wr: usize) -> Self {
        Self { 
            delay_buffer: msgf_afrm::AudioFrame::new(44100,44100),
            rd_ptr: 0, wr_ptr: wr,
        }
    }
}
impl SdDelay {
    pub fn new(ref_prms: &msgf_delay::DelayParameter) -> Self {
        let wrl: usize = (ref_prms.l_time*44100.0) as usize;
        let wrr: usize = (ref_prms.r_time*44100.0) as usize;
        SdDelay {
            att_ratio: ref_prms.att_ratio,
            dbuf: [SingleBuf::init(wrl), SingleBuf::init(wrr)],
        }
    }
    fn inc_ptr(&mut self, str: usize) {
        self.dbuf[str].rd_ptr += 1;
        self.dbuf[str].wr_ptr += 1;
        if self.dbuf[str].rd_ptr >= 44100 {
            self.dbuf[str].rd_ptr = 0;
        }
        if self.dbuf[str].wr_ptr >= 44100 {
            self.dbuf[str].wr_ptr = 0;
        }
    }
}
impl Engine for SdDelay {
    fn process_as2(&mut self, 
        in_abuf: [&mut msgf_afrm::AudioFrame;2], 
        out_abuf: [&mut msgf_afrm::AudioFrame;2]) {
        let snum = in_abuf[0].sample_number;
        for str in 0..2 {
           