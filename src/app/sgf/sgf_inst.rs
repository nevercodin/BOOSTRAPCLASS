
//
//  sgf_inst.rs
//	Musical Sound Generator Framework
//      Sing Instrument Class
//
//  Created by Hasebe Masahiko on 2022/06/11.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use std::rc::Rc;
use std::cell::Cell;
use crate::msgf_if;
use crate::core::*;
use crate::core::msgf_voice::*;
use crate::core::msgf_disp::MsgfDisplay;
//use crate::engine::*;
use crate::app::sgf::*;
use crate::app::sgf::sgf_voice;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
struct NoteSgf {
    note: u8,
    _vel: u8,
    off: bool,
}
pub struct InstSgf {
    vce_audio: msgf_afrm::AudioFrame,
    inst_audio: msgf_afrm::AudioFrame,
    vcevec: Vec<NoteSgf>,                    // 押鍵中の Note
    vce: Option<Box<sgf_voice::VoiceSgf>>,    // 発音中の Voice
    active_vce_index: i8,                  // 発音中の vcevec のIndex
    inst_number: usize,
    mdlt: f32,  //  0.0..0.5