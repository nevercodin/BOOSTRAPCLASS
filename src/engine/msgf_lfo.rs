
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
        Lfo {
            fadein_time: ref_prms.fadein_time,
            delay_time: ref_prms.delay_time,
            next_phase: 0.0,
            delta_phase: Lfo::calc_freq(ref_prms.freq),
            direction: coef.4,
            x1: coef.0,
            x2: coef.1,
            y: coef.2,
            z: coef.3,
            dac_counter: 0,
        }
    }
    fn calc_wave(wv: LfoWave, dir: LfoDirection) -> (f32, f32, f32, f32, LfoDirection) {
        let (x1, x2, y, z): (f32, f32, f32, f32);
        match wv {
            LfoWave::Tri => {x1=0.5; x2=1.5; y=4.0; z=0.0;}
            LfoWave::Saw => {x1=0.0; x2=2.0; y=2.0; z=0.0;}
            LfoWave::Squ => {x1=0.5; x2=1.5; y=100000.0; z=0.0;}
            LfoWave::Sin => {x1=0.5; x2=1.5; y=2.0*msgf_if::PI; z=1.0/6.78;}
        };
        (x1, x2, y, z, dir)
    }
    fn calc_freq(freq: f32) -> f32 {
        freq*(msgf_if::AUDIO_FRAME_PER_CONTROL as f32)/msgf_if::SAMPLING_FREQ
    }
    pub fn set_wave(&mut self, value: u8) {
        let dir_num: u8 = value&0x30;
        let dir: LfoDirection;
        match dir_num {
            0 => dir = LfoDirection::LfoBoth,
            1 => dir = LfoDirection::LfoUpper,
            2 => dir = LfoDirection::LfoLower,