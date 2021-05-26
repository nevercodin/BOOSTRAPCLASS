//
//  va_inst.rs
//	Musical Sound Generator Framework
//      Virtual Analog Instrument Class
//
//  Created by Hasebe Masahiko on 2021/11/21.
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
use crate::engine::msgf_gen::Engine;
use crate::app::va::*;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
const MAX_PB_RANGE:f32 = 1200.0;
const MIDI_MAX_PB_VAL:f32 = 8192.0;
const MIDI_CENTER_VAL:f32 = 64.0;
//---------------------------------------------------------
pub struct InstVa {
    vce_audio: msgf_afrm::AudioFrame,
    inst_audio: msgf_afrm::AudioFrame,
    vcevec: Vec<va_voice::VoiceVa>,
    delay: msgf_delay::Delay,
    inst_number: usize,
    mdlt: f32,  //  0.0..0.5
    pit: f32,   //  [cent]
    vol: u8,    //  0..127
    pan: f32,   //  -1..0..+1
    exp: u8,    //  0..127
    inst_prm: Rc<Cell<va_prm::SynthParameter>>,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl MsgfDisplay for InstVa {}
impl Drop for InstVa {
    fn drop(&mut self) {self.vcevec.clear();}
}
//---------------------------------------------------------
impl msgf_inst::Inst for InstVa {
/*
    fn new(inst_number: usize, vol: u8, pan: u8, exp: u8) -> Self {
        let max_tone = va_prm::MAX_TONE_COUNT;
        let mut inst_num = inst_number;
        if inst_number >= max_tone {
            inst_num = max_tone-1;
        }
        let prm = Rc::new(Cell::new(va_prm::TONE_PRM[inst_number]));
        Self {
            vce_audio: msgf_afrm::AudioFrame::new(0,msgf_if::MAX_BUFFER_SIZE),
            inst_audio: msgf_afrm::AudioFrame::new(0,msgf_if::MAX_BUFFER_SIZE),
            vcevec: Vec::new(),
            delay: msgf_delay::Delay::new(&prm.get().delay),
            inst_number: inst_num,
            mdlt: prm.get().osc.lfo_depth,
            pit: 0.0,
            vol,
           