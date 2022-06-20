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
    wr_ptr: [