
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