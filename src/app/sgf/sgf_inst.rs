
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
    pit: f32,   //  [cent]
    vol: u8,    //  0..127
    pan: f32,   //  -1..0..+1
    exp: u8,    //  0..127
    spmsg: [u8;4],    //  special message for SGF [1st Fmnt, 2nd Fmnt]
    inst_prm: Rc<Cell<sgf_prm::SynthParameter>>,
}
const NO_NOTE:i8 = -1;
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl MsgfDisplay for InstSgf {}
impl NoteSgf {
    fn new(note: u8, _vel: u8) -> Self {
        Self {note, _vel, off: false,}
    }
}
impl Drop for InstSgf {
    fn drop(&mut self) {self.vcevec.clear();}
}
//---------------------------------------------------------
impl msgf_inst::Inst for InstSgf {

    fn change_inst(&mut self, mut inst_number: usize, vol: u8, pan: u8, exp: u8) {
        let max_tone = sgf_prm::SGF_MAX_TONE_COUNT;
        if inst_number >= max_tone {
            inst_number = max_tone-1;
        }
        let _ = &self.inst_prm.replace(sgf_prm::SGF_TONE_PRM[inst_number]);
        self.inst_number = inst_number;
        self.mdlt = self.inst_prm.get().osc.lfo_depth;
        self.pit = 0.0;
        self.vol = vol;
        self.pan = Self::calc_pan(pan);
        self.exp = exp;
    }
    fn note_off(&mut self, dt2: u8, _dt3: u8) {
        let nt_idx = self.search_note(dt2);
        if nt_idx == NO_NOTE {return}
        if nt_idx == self.active_vce_index {
            // sounding voice
            if let Some(cur_vce) = &mut self.vce {
                cur_vce.note_off();
            }
            self.vcevec[nt_idx as usize].off = true;
        }
        else {
            // key pressd, but already no sound
            self.remove_note(nt_idx);
        }
    }
    fn note_on(&mut self, dt2: u8, dt3: u8) {
        if let Some(cur_vce) = &mut self.vce {
            cur_vce.slide(dt2, dt3);
        }
        else {// 1st Note On
            let mut new_vce = Box::new(
                sgf_voice::VoiceSgf::new(dt2, dt3, 
                    self.mdlt, self.pit, self.vol, self.exp, Rc::clone(&self.inst_prm)));
            // Send Special Message to new voice
            for i in 0..self.spmsg.len() {  //  Formant のみ
                new_vce.set_prm(i as u8, self.spmsg[i]);
            }
            new_vce.start_sound();
            self.vce = Some(new_vce);
        }
        let cur_note = self.active_vce_index;
        if cur_note > NO_NOTE && self.vcevec[cur_note as usize].off == true {
            // Same note is releasing now
            self.remove_note(cur_note);
        }
        self.vcevec.push(NoteSgf::new(dt2, dt3));
        self.active_vce_index = (self.vcevec.len() as i8)-1; // the last order
    }
    fn per_note_after(&mut self, note: u8, val: u8) {
        let nt_idx = self.search_note(note);
        if nt_idx == NO_NOTE {return}
        if nt_idx == self.active_vce_index {
            // sounding voice
            if let Some(cur_vce) = &mut self.vce {
                let mdlt: f32 = InstSgf::calc_pmd(val);
                self.mdlt = mdlt;
                cur_vce.change_pmd(mdlt);
            }
            self.vcevec[nt_idx as usize].off = true;
        }
    }
    fn modulation(&mut self, value: u8) {
        let mdlt = InstSgf::calc_pmd(value);
        self.mdlt = mdlt;
        if let Some(cur_vce) = &mut self.vce {
            cur_vce.change_pmd(mdlt);
        }
    }
    fn volume(&mut self, value: u8) {
        self.vol = value;
        let exp = self.exp;
        if let Some(cur_vce) = &mut self.vce {
            cur_vce.amplitude(value, exp);
        }
    }
    fn pan(&mut self, value: u8) {
        self.pan = Self::calc_pan(value);
    }
    fn expression(&mut self, value: u8) {
        self.exp = value;
        let vol = self.vol;
        if let Some(cur_vce) = &mut self.vce {
            cur_vce.amplitude(vol, value);
        }
    }
    fn pitch(&mut self, bend:i16, tune_coarse:u8, tune_fine:u8) {
        let pit:f32 = ((bend as f32)*200.0)/8192.0
                     + ((tune_coarse as f32)-64.0)*100.0
                     + ((tune_fine as f32)-64.0)*100.0/64.0;
        self.pit = pit;
        if let Some(cur_vce) = &mut self.vce {
            cur_vce.pitch(pit);
        }
    }
    fn sustain(&mut self, _value: u8) {}
    fn all_sound_off(&mut self) {
        if let Some(cur_vce) = &mut self.vce {
            cur_vce.damp();
        }
    }
    fn set_prm(&mut self, prm_type: u8, value: u8) {
        let idx: usize = prm_type as usize;