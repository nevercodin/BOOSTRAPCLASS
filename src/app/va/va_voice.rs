
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
}
//---------------------------------------------------------
impl msgf_voice::Voice for VoiceVa {
    fn start_sound(&mut self) {
        self.aeg.move_to_attack();
        self.lfo.start();
    }
    fn note_off(&mut self) {
        self.status = NoteStatus::AfterNoteOff;
        self.aeg.move_to_release()
    }
    fn note_num(&self) -> u8 {self.note}
    fn velocity(&self) -> u8 {self.vel}
    fn change_pmd(&mut self, value: f32) {
        self.osc.change_pmd(value);
        self.emphasis_vol = 1.0 + value*6.0; // 1.0 - 1.5
    }
    fn amplitude(&mut self, volume: u8, expression: u8) {
        self.max_note_vol = VoiceVa::calc_vol(volume, expression);
    }
    fn pitch(&mut self, pitch:f32) {
        self.osc.change_pitch(pitch);
    }
    fn status(&self) -> NoteStatus {self.status}
    fn damp(&mut self) {
        self.status = NoteStatus::DuringDamp;
        self.damp_counter = 0;
    }
    fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, in_number_frames: usize) -> bool {
        if self.ended {return self.ended;}

        //  Pitch Control
        let cbuf_size = msgf_cfrm::CtrlFrame::get_cbuf_size(in_number_frames);
        let lbuf = &mut msgf_cfrm::CtrlFrame::new(cbuf_size);

        //  LFO
        self.lfo.process_c(lbuf);

        //  Oscillator
        self.osc.process_ac(abuf, lbuf);

        //  AEG
        let aegbuf = &mut msgf_cfrm::CtrlFrame::new(cbuf_size);
        self.aeg.process_c(aegbuf);

        //  Volume
        for i in 0..abuf.sample_number {
            let aeg = aegbuf.ctrl_for_audio(i);
            abuf.mul_rate(i, self.max_note_vol*self.emphasis_vol*aeg);
        }
        msgf_voice::manage_note_level(self, abuf, aegbuf)
    }
    fn set_prm(&mut self, prm_type: u8, value: u8) {
        match prm_type {
            0 => self.lfo.set_freq(value),
            1 => self.lfo.set_wave(value),
            _ => ()
        }
    }
    fn put_lvl_check_buf(&mut self, lvl: f32) {self.lvl_check_buf.put_into_abuf(lvl);}
    fn damp_counter(&self) -> u32 {self.damp_counter}
    fn inc_damp_counter(&mut self) {self.damp_counter+=1;}
    fn ended(&self) -> bool {self.ended}
    fn set_ended(&mut self, which: bool) {self.ended = which;}
}

impl VoiceVa {
    pub fn new(note:u8, vel:u8, pmd:f32, pit:f32, vol:u8, exp:u8,
        inst_prm: Rc<Cell<va_prm::SynthParameter>>) -> Self {
        let tprm: &va_prm::SynthParameter = &inst_prm.get();
        Self {
            note,
            vel,
            status: NoteStatus::DuringNoteOn,
            damp_counter: 0,
            lvl_check_buf: msgf_afrm::AudioFrame::new((msgf_if::SAMPLING_FREQ/100.0) as usize, msgf_if::MAX_BUFFER_SIZE),
            osc: msgf_osc::Osc::new(&tprm.osc, note, pmd, pit),
            aeg: msgf_aeg::Aeg::new(&tprm.aeg),
            lfo: msgf_lfo::Lfo::new(&tprm.lfo),
            max_note_vol: VoiceVa::calc_vol(vol, exp),
            emphasis_vol: 1.0,
            ended: false,
        }
    }
    fn calc_vol(vol:u8, exp:u8) -> f32 {
        let exp_sq = exp as f32;
        let vol_sq = vol as f32;
        let total_vol = 0.5f32.powf(4.0);    // 4bit margin
        (total_vol*vol_sq*exp_sq)/16384.0
    }
}