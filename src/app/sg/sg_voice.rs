
//
//  sg_voice.rs
//	Musical Sound Generator Framework
//      Sing Voice Class
//
//  Created by Hasebe Masahiko on 2022/03/15.
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
use crate::engine::*;
use crate::engine::msgf_gen::Engine;
use crate::app::sg::*;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct VoiceSg {
    // Note
    note: u8,
    vel: u8,
    status: NoteStatus,
    damp_counter: u32,
    lvl_check_buf: msgf_afrm::AudioFrame,
    // Synth
    osc: msgf_additive::Additive,
    aeg: msgf_aeg::Aeg,
    lfo: msgf_lfo::Lfo,
    max_note_vol: f32,
    ended: bool,
    vowel_x: f32,   // -1..0..1
    vowel_y: f32,   // -1..0..1
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl MsgfDisplay for VoiceSg {}
impl PartialEq for VoiceSg {
    fn eq(&self, other: &Self) -> bool {
        self.note == other.note && self.vel == other.vel
    }
}
//---------------------------------------------------------
impl msgf_voice::Voice for VoiceSg {
    fn start_sound(&mut self) {
        self.aeg.move_to_attack();
        self.lfo.start();
    }
    fn slide(&mut self, note:u8, vel:u8) {
        self.note = note;
        self.vel = vel;
        self.status = NoteStatus::DuringNoteOn;
        self.damp_counter = 0;
        self.osc.change_note(note);
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
    }
    fn amplitude(&mut self, volume: u8, expression: u8) {
        self.max_note_vol = VoiceSg::calc_vol(volume, expression);
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
            abuf.mul_rate(i, self.max_note_vol*aeg);
        }
        msgf_voice::manage_note_level(self, abuf, aegbuf)
    }
    fn set_prm(&mut self, prm_type: u8, value: u8) {
        match prm_type {