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
i