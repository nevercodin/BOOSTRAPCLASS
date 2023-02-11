
//
//  msgf_vocal.rs
//	Musical Sound Generator Framework
//      Vocal Class
//      声帯振動による波形を生成する
//
//  Created by Hasebe Masahiko on 2022/06/11.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;
use crate::engine::msgf_gen;
use crate::engine::msgf_gen::*;
//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(Copy, Clone)]
pub struct VocalParameter {
    pub coarse_tune: i32,   //  [semitone]
    pub fine_tune: f32,     //  [cent]
    pub lfo_depth: f32,     //  1.0 means +-1oct.
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Vocal {
    prms_variable: VocalParameter,
    pmd: f32,           //  1.0: 1200[cent]
    base_pitch: f32,    //  [Hz]
    cnt_ratio: f32,     //  ratio of Hz
    next_phase: f32,    //  0.0 - 1.0
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Vocal {
    pub fn new(prms:&VocalParameter, note:u8, pmd:f32, cnt_pitch:f32) -> Vocal {
        Vocal {
            prms_variable: *prms,
            pmd,
            base_pitch: Vocal::calc_base_pitch(prms.coarse_tune, prms.fine_tune, note),
            cnt_ratio: Vocal::calc_cnt_pitch(cnt_pitch),
            next_phase: 0.0,
        }
    }
    pub fn change_pmd(&mut self, value:f32) {self.pmd = value;}
    pub fn change_note(&mut self, note:u8) {
        self.base_pitch = Vocal::calc_base_pitch(self.prms_variable.coarse_tune,
                                            self.prms_variable.fine_tune, note);
    }
    pub fn limit_note(calculated_note:i32) -> u8 {
        let mut note = calculated_note;
        while note < 0 { note += 12;}
        while note >= 128 { note -= 12;}
        note as u8
    }
    pub fn calc_base_pitch(coarse_tune:i32, fine_tune:f32, note:u8) -> f32 {
        let tune_note: u8 = Vocal::limit_note(note as i32 + coarse_tune);