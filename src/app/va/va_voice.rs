
//
//  va_voice.rs
//	Musical Sound Generator Framework
//      Virtual Analog Voice Class
//
//  Created by Hasebe Masahiko on 2021/09/18.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use std::rc::Rc;
use std::cell::Cell;
use crate::msgf_if;
use crate::core::*;
use crate::core::msgf_voice::*;
use crate::core::msgf_disp::MsgfDisplay;
use crate::engine::*;
use crate::engine::msgf_gen::*;
use crate::app::va::*;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct VoiceVa {
    // Note
    note: u8,
    vel: u8,
    status: NoteStatus,
    damp_counter: u32,
    lvl_check_buf: msgf_afrm::AudioFrame,
    // Synth
    osc: msgf_osc::Osc,
    aeg: msgf_aeg::Aeg,
    lfo: msgf_lfo::Lfo,
    max_note_vol: f32,
    emphasis_vol: f32,
    ended: bool,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl MsgfDisplay for VoiceVa {}
impl PartialEq for VoiceVa {
    fn eq(&self, other: &Self) -> bool {
        self.note == other.note && self.vel == other.vel
    }