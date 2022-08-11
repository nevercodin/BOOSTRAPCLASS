
//
//  msgf_lfo.rs
//	Musical Sound Generator Framework
//      LFO Class
//
//  Created by Hasebe Masahiko on 2021/10/17.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;
use crate::engine::msgf_gen::*;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum LfoDirection {
    LfoBoth,
    LfoUpper,
    LfoLower,
}
#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum LfoWave {
    Tri,
    Saw,
    Squ,
    Sin,
}
#[derive(Copy, Clone)]
pub struct LfoParameter {
    pub freq: f32,              // RTP, prm#0
    pub wave: LfoWave,          // NKP, prm#1:bit 7-6
    pub direction: LfoDirection,// NKP, prm#1:bit 2-0
    pub fadein_time: u64,       // NKP, prm#2
    pub delay_time: u64,        // NKP, prm#3
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Lfo {
    fadein_time: u64,
    delay_time: u64,
    next_phase: f32,
    delta_phase: f32,
    direction: LfoDirection,
    x1: f32,
    x2: f32,
    y: f32,
    z: f32,
    dac_counter: u64,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Lfo {
    pub fn new(ref_prms: &LfoParameter) -> Lfo {
        let coef = Lfo::calc_wave(ref_prms.wave, ref_prms.direction);