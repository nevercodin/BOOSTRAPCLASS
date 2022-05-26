
//
//  msgf_part.rs
//	Musical Sound Generator Framework
//      Part Class
//
//  Created by Hasebe Masahiko on 2021/09/16.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::*;
use crate::core::*;
use crate::core::msgf_disp::MsgfDisplay;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Part {
    //	Part Latest Parameter Value
    cc0_msb: u8,
    cc1_modulation_wheel: u8,
    cc5_portamento_time: u8,
    cc7_volume: u8,
    cc10_pan: u8,
    cc11_expression: u8,
    cc12_note_shift: u8,	//	out of MIDI
    cc13_tune: u8,			//	out of MIDI
    cc32_lsb: u8,
    cc64_sustain: u8,
    cc65_portamento: u8,
    cc66_sostenuto: u8,
    cc91_revsend: u8,
    _cc126_mono: u8,
    program_number: u8,
    pitch_bend_value: i16,
    cc16_31_change_vprm: [u8; 16],

    //	Composite Object
    inst: Box<dyn msgf_inst::Inst>,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl MsgfDisplay for Part {}
impl Part {
    pub fn new() -> Self {
        Self {
            cc0_msb: 0,
            cc1_modulation_wheel: 0,
            cc5_portamento_time: 0,
            cc7_volume: 100,
            cc10_pan: 64,
            cc11_expression: 127,
            cc12_note_shift: 64,
            cc13_tune: 64,
            cc32_lsb: 0,
            cc64_sustain: 0,
            cc65_portamento: 0,
            cc66_sostenuto: 0,
            cc91_revsend: 127,
            _cc126_mono: 1,
            program_number: 0,
            pitch_bend_value: 0,
            cc16_31_change_vprm: [0; 16],
            inst: app::get_inst(0,100,64,127), //pgn,vol,pan,exp,
        }
    }
    pub fn note_off(&mut self, dt2: u8, dt3: u8) {
        self.inst.note_off(dt2, dt3)
    }
    pub fn note_on(&mut self, dt2: u8, dt3: u8) {
        self.inst.note_on(dt2, dt3)
    }
    pub fn per_note_after(&mut self, dt2: u8, dt3: u8) {
        self.inst.per_note_after(dt2, dt3)
    }
    pub fn control_change(&mut self, controller: u8, value: u8) {
        match controller {
            0 => self.cc0_msb = value,
            1 => {
                self.cc1_modulation_wheel = value;
                self.inst.modulation(value);
            }
            5 => self.cc5_portamento_time = value,
            7 => {
                self.cc7_volume = value;
                self.inst.volume(value);
            }
            10 => {
                self.cc10_pan = value;
                self.inst.pan(value);
            }
            11 => {
                self.cc11_expression = value;
                self.inst.expression(value);
            }
            12 => {
                self.cc12_note_shift = value;
                let pb = self.pitch_bend_value;
                let tn = self.cc13_tune;
                self.inst.pitch(pb, value, tn);
            }
            13 => {
                self.cc13_tune = value;
                let pb = self.pitch_bend_value;
                let ns = self.cc12_note_shift;
                self.inst.pitch(pb, ns, value);
            }
            32 => self.cc32_lsb = value,
            64 => {