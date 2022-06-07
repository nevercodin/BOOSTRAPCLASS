
//
//  msgf_aeg.rs
//	Musical Sound Generator Framework
//      Aeg Class
//
//  Created by Hasebe Masahiko on 2021/10/15.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::*;
use crate::engine::msgf_gen::*;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(Copy, Clone)]
pub struct AegParameter {
    pub attack_rate: f32,
    pub decay_rate: f32,
    pub sustain_level: f32,
    pub release_rate: f32,
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
#[derive(PartialEq, Clone, Copy)]
pub enum EgState {
    NotYet,
    Attack,     //  A
    Decay,      //  D
    Sustain,    //  S
    Release,    //  R
    EgDone,
    _Damp,
}
pub struct Aeg {
    prms: AegParameter,
    state: EgState,
    tgt_value: f32,
    src_value: f32,
    crnt_value: f32,
    crnt_rate: f32,
    interpolate_value: f32,
    release_rsv: bool,
    last_value: f32,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Aeg {
    pub fn new(ref_prms: &AegParameter) -> Aeg {
        Aeg {
            prms: *ref_prms,
            state: EgState::NotYet,
            tgt_value: 0.0,
            src_value: 0.0,
            crnt_value: 0.0,
            crnt_rate: 1.0,
            interpolate_value: 0.0,
            release_rsv: false,
            last_value: 0.0,
        }
    }
    pub fn move_to_attack(&mut self) {
        self.src_value = 0.0;
        self.tgt_value = 1.0;
        self.crnt_rate = self.prms.attack_rate;
        self.state = EgState::Attack;
        self.interpolate_value = 0.0;
        self.release_rsv = false;
    }
    fn move_to_decay(&mut self, eg_crnt: f32) {
        if self.prms.decay_rate == 1.0 {
            self.move_to_sustain(eg_crnt);
        } else {
            self.src_value = eg_crnt;
            self.tgt_value = self.prms.sustain_level;
            self.crnt_rate = self.prms.decay_rate;
            self.state = EgState::Decay;
            self.interpolate_value = 0.0;
        }
    }
    fn move_to_sustain(&mut self, eg_crnt: f32) {
        if self.prms.sustain_level == 0.0 {
            self.move_to_egdone();
        } else {
            self.src_value = eg_crnt;
            self.tgt_value = self.prms.sustain_level;
            self.crnt_rate = 0.0;
            self.state = EgState::Sustain;
            self.interpolate_value = 0.0;
        }
    }