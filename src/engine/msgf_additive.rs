
//
//  msgf_additive.rs
//	Musical Sound Generator Framework
//      Additive Class
//
//  Created by Hasebe Masahiko on 2022/03/21.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;
use crate::engine::msgf_gen;
use crate::engine::msgf_gen::*;
use crate::engine::msgf_osc::*;
//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(Copy, Clone)]
pub struct AdditiveParameter {
    pub coarse_tune: i32,   //  [semitone]
    pub fine_tune: f32,     //  [cent]
    pub pmd: f32,
    pub prtm_spd: f32,      //  speed of portamento: 0.0(fastest)-?
    pub magnitude: f32,
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Additive {
    prms_variable: AdditiveParameter,
    pmd: f32,
    base_pitch: f32,    //  [Hz]
    next_phase: f32,    //  0.0 - 1.0
    //  for Portamento
    target_pitch: f32,  //  [Hz]
    real_prtm_spd: f32,
    //  for Pitch Bend
    cnt_ratio: f32,     //  ratio of Hz
    //  Formant
    f1: f32,
    f2: f32,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
//      f1      f2      f3      f4
//  a   800     1200    2500    3500
//  i   300     2300    2900    3500
//  u   300     1200    2500    3500
//  e   500     1900    2500    3500
//  o   500     800     2500    3500
impl Additive {
    pub fn new(prms:&AdditiveParameter, note:u8, cnt_pitch:f32) -> Additive {
        let pit = Osc::calc_base_pitch(prms.coarse_tune, prms.fine_tune, note);
        Self {
            prms_variable: *prms,
            pmd: prms.pmd,
            base_pitch: pit,
            next_phase: 0.0,
            target_pitch: pit,
            real_prtm_spd: 0.0,
            cnt_ratio: Osc::calc_cnt_pitch(cnt_pitch),
            f1: 300.0,
            f2: 2300.0,
        }
    }
    pub fn change_pmd(&mut self, value:f32) {self.pmd = value;}
    pub fn change_note(&mut self, note:u8) {
        self.target_pitch = Osc::calc_base_pitch(self.prms_variable.coarse_tune,
                                                 self.prms_variable.fine_tune, note);
        self.base_pitch += (self.target_pitch-self.base_pitch)/2.0; // 一気に真ん中まで
        self.real_prtm_spd = self.prms_variable.prtm_spd;//(diff_note as f32).powf(0.25);
    }
    pub fn change_pitch(&mut self, cnt_pitch:f32) {