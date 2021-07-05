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
            pan: Self::calc_pan(pan),
            exp,
            inst_prm: prm,
        }
    }
*/
    fn change_inst(&mut self, mut inst_number: usize, vol: u8, pan: u8, exp: u8) {
        let max_tone = va_prm::MAX_TONE_COUNT;
        if inst_number >= max_tone {
            inst_number = max_tone-1;
        }
        let _ = &self.inst_prm.replace(va_prm::TONE_PRM[inst_number]);
        self.delay = msgf_delay::Delay::new(&self.inst_prm.get().delay);
        self.inst_number = inst_number;
        self.mdlt = self.inst_prm.get().osc.lfo_depth;
        self.pit = 0.0;
        self.vol = vol;
        self.pan = Self::calc_pan(pan);
        self.exp = exp;
    }
    fn note_off(&mut self, dt2: u8, _dt3: u8) {
        let nt_opt = self.search_note(dt2, NoteStatus::DuringNoteOn);
        if let Some(nt) = nt_opt {
            nt.note_off();
        }
    }
    fn note_on(&mut self, dt2: u8, dt3: u8) {
        let mut new_voice = va_voice::VoiceVa::new(
            dt2, dt3, self.mdlt, self.pit, self.vol, self.exp, Rc::clone(&self.inst_prm)
        );
        new_voice.start_sound();
        self.vcevec.push(new_voice);
    }
    fn per_note_after(&mut self, dt2: u8, dt3: u8) {
        let nt = self.search_note( dt2, NoteStatus::DuringNoteOn);
        if let Some(vce) = nt {
            let pmd = (dt3 as f32)/(127.0*12.0);    // MAX+-100[cent]
            vce.change_pmd(pmd);    //  0 - 1/12
        }
    }
    fn modulation(&mut self, value: u8) {
        let mdlt = 0.5f32*(value as f32)/127.0; // 0.0 - 0.5
        self.mdlt = mdlt;
        self.vcevec.iter_mut().for_each(|vce| vce.change_pmd(mdlt));
    }
    fn volume(&mut self, value: u8) {
        self.vol = value;
        let exp = self.exp;
        self.vcevec.iter_mut().for_each(|vce| vce.amplitude(value, exp));
    }
    fn pan(&mut self, value: u8) {
        self.pan = Self::calc_pan(value);
    }
    fn expression(&mut self, value: u8) {
        self.exp = value;
        let vol = self.vol;
        self.vcevec.iter_mut().for_each(|vce| vce.amplitude(vol, value));
    }
    fn pitch(&mut self, bend:i16, tune_coarse:u8, tune_fine:u8) {
        let pit:f32 = ((bend as f32)*MAX_PB_RANGE)/MIDI_MAX_PB_VAL
            + ((tune_coarse as f32)-MIDI_CENTER_VAL)*100.0 
            + ((tune_fine as f32)-MIDI_CENTER_VAL)*100.0/MIDI_CENTER_VAL;
        self.pit = pit;
        self.vcevec.iter_mut().for_each(|vce| vce.pitch(pit));
    }
    fn sustain(&mut self, _value: u8) {}
    fn a