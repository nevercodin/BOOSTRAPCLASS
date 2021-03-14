
//
//  sgf_voice.rs
//	Musical Sound Generator Framework
//      Sing Voice Class
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
use crate::engine::*;
use crate::engine::msgf_gen::Engine;
use crate::app::sgf::*;
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
const DEFAULT_F1:f32 = 800.0;
const DEFAULT_F2:f32 = 1200.0;
const BPF_RESO:f32 = 3.0;
const NOTE_OFFSET:u8 = 24;
//---------------------------------------------------------
pub struct VoiceSgf {
    // Note
    note: u8,
    vel: u8,
    status: NoteStatus,
    damp_counter: u32,
    lvl_check_buf: msgf_afrm::AudioFrame,
    // Synth
    vcl: msgf_vocal::Vocal,
    lpf: msgf_biquad::Biquad,
    frm1: msgf_biquad::Biquad,
    frm2: msgf_biquad::Biquad,
    aeg: msgf_aeg::Aeg,
    lfo: msgf_lfo::Lfo,
    max_note_vol: f32,
    ended: bool,
    vowel_x: f32,   // -1..0..1
    vowel_y: f32,   // -1..0..1
    fmnt_adjust_vol: f32,   //  0..1
    scl_adjust_vol: f32,    // 0..1
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl MsgfDisplay for VoiceSgf {}
impl PartialEq for VoiceSgf {
    fn eq(&self, other: &Self) -> bool {
        self.note == other.note && self.vel == other.vel
    }
}
//---------------------------------------------------------
impl msgf_voice::Voice for VoiceSgf {
    fn start_sound(&mut self) {
        self.aeg.move_to_attack();
        self.lpf.set_thru();
        self.frm1.set_bpf(DEFAULT_F1, BPF_RESO);
        self.frm2.set_bpf(DEFAULT_F2, BPF_RESO);
        self.lfo.start();
    }
    fn slide(&mut self, note:u8, vel:u8) {
        let real_note = note - NOTE_OFFSET;
        self.note = real_note;
        self.vel = vel;