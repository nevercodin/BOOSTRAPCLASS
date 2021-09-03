
//
//  va_prm.rs
//	Musical Sound Generator Framework
//      Instruments Parameter
//
//  Created by Hasebe Masahiko on 2021/10/25.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::engine::msgf_osc::*;
use crate::engine::msgf_aeg::*;
use crate::engine::msgf_lfo::*;
use crate::engine::msgf_delay::*;

#[derive(Copy, Clone)]
pub struct SynthParameter {
    pub osc: OscParameter,
    pub aeg: AegParameter,
    pub lfo: LfoParameter,
    pub delay: DelayParameter,
}

pub const MAX_TONE_COUNT:usize = 6;
pub const TONE_PRM: [SynthParameter; MAX_TONE_COUNT] = [
    // No.0
    SynthParameter {
        osc: OscParameter {
            coarse_tune: 0,     //  i32 : 0 means tuning of A=440[Hz]
            fine_tune: 0.0,     //  f32 : 1.0 means 1[cent]
            lfo_depth: 0.02,    //  f32 : 1.0 means +-1oct.
            wv_type: WvType::Sine,
        },
        aeg: AegParameter {
            attack_rate: 0.9,   //  0.0-1.0
            decay_rate: 0.2,    //  0.0-1.0 : 1.0 means no decay and no sustain level
            sustain_level: 0.0, //  1 means same value as Attack Level
            release_rate: 0.01, //  0.0-1.0
        },
        lfo: LfoParameter {
            freq: 2.0,          //  [Hz]
            wave: LfoWave::Tri,
            direction: LfoDirection::LfoBoth,
            fadein_time: 30,   //  1: AUDIO_FRAME_PER_CONTROL / SAMPLING_FREQ (=3msec)
            delay_time: 0,    //    same as above
        },
        delay: DelayParameter {
            l_time: 0.5,        //  0.0 - 1.0 [sec]
            r_time: 0.5,        //  0.0 - 1.0 [sec]
            att_ratio: 0.4,     //  attenuation
        },
    },
    // No.1
    SynthParameter {
        osc: OscParameter {