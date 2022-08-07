
//
//  msgf_gen.rs
//	Musical Sound Generator Framework
//      engine Trait / Tables
//
//  Created by Hasebe Masahiko on 2022/08/23.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::*;
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub trait Engine {
    fn process_a(
        &mut self, 
        _abuf: &mut msgf_afrm::AudioFrame) {}
    fn process_as(
        &mut self, 
        _in_abuf: [&mut msgf_afrm::AudioFrame;2]) {}
    fn process_c(
        &mut self,
        _cbuf: &mut msgf_cfrm::CtrlFrame) {}
    fn process_ac(
        &mut self,
        _abuf: &mut msgf_afrm::AudioFrame,