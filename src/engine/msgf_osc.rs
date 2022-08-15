
//
//  msgf_osc.rs
//	Musical Sound Generator Framework
//      Osc Class
//
//  Created by Hasebe Masahiko on 2021/10/15.
//  Copyright (c) 2021 Hasebe Masahiko.
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
#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum WvType {
    Sine,
    Saw,
    Square,
    Pulse,
}
#[derive(Copy, Clone)]
pub struct OscParameter {
    pub coarse_tune: i32,   //  [semitone]
    pub fine_tune: f32,     //  [cent]
    pub lfo_depth: f32,     //  1.0 means +-1oct.
    pub wv_type: WvType,
}
type WvFn = fn(f32, usize) -> f32;
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Osc {
    prms_variable: OscParameter,
    pmd: f32,
    base_pitch: f32,    //  [Hz]
    cnt_ratio: f32,     //  ratio of Hz
    next_phase: f32,    //  0.0 - 1.0
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Osc {
    pub fn new(prms:&OscParameter, note:u8, pmd:f32, cnt_pitch:f32) -> Osc {
        Osc {
            prms_variable: *prms,
            pmd,
            base_pitch: Osc::calc_base_pitch(prms.coarse_tune, prms.fine_tune, note),
            cnt_ratio: Osc::calc_cnt_pitch(cnt_pitch),
            next_phase: 0.0,
        }
    }