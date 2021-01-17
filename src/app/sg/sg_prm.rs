//
//  sg_prm.rs
//	Musical Sound Generator Framework
//      Instruments Parameter
//
//  Created by Hasebe Masahiko on 2022/03/15.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::engine::msgf_additive::*;
use crate::engine::msgf_aeg::*;
use crate::engine::msgf_lfo::*;

#[derive(Copy, Clone)]
pub struct SynthParameter {
    pub osc: AdditiveParameter,
    pub aeg: AegParameter,
    pub lfo: LfoPar