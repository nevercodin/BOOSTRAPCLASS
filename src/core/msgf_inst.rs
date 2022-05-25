
//
//  msgf_inst.rs
//	Musical Sound Generator Framework
//      Instrument Trait
//
//  Created by Hasebe Masahiko on 2021/09/18.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::*;
pub trait Inst {
    //fn new(inst_number: usize, vol: u8, pan: u8, exp: u8) -> Self;
    fn change_inst(&mut self, inst_number: usize, vol: u8, pan: u8, exp: u8);
    fn note_off(&mut self, dt2: u8, dt3: u8);
    fn note_on(&mut self, dt2: u8, dt3: u8);
    fn per_note_after(&mut self, _dt2: u8, _dt3: u8){}  // Default Implementations
    fn modulation(&mut self, _value: u8){}              // Default Implementations
    fn volume(&mut self, value: u8);
    fn pan(&mut self, _value: u8){}                     // Default Implementations
    fn expression(&mut self, _value: u8){}              // Default Implementations
    fn pitch(&mut self, bend:i16, tune_coarse:u8, tune_fine:u8);
    fn sustain(&mut self, _value: u8){}                 // Default Implementations
    fn all_sound_off(&mut self);
    fn set_prm(&mut self, _prm_type: u8, _value: u8){}  // prm_type:0-15, value:0-127
    //fn release_note(&mut self, nt: &msgf_voice::Voice);
    fn process(&mut self,
        abuf_l: &mut msgf_afrm::AudioFrame,
        abuf_r: &mut msgf_afrm::AudioFrame,
        in_number_frames: usize);
}