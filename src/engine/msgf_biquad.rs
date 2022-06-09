
//
//  msgf_biquad.rs
//	Musical Sound Generator Framework
//      Biquad Class
//
//  Created by Hasebe Masahiko on 2022/06/04.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;
use crate::engine::msgf_gen::*;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(Copy, Clone)]
pub struct BiquadParameter {
    pub freq: f32,
    pub resonance: f32,
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Biquad {