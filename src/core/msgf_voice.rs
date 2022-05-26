
//
//  msgf_voice.rs
//	Musical Sound Generator Framework
//      Voice Class
//
//  Created by Hasebe Masahiko on 2021/09/18.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::*;
use crate::msgf_if;
use crate::core::msgf_disp::MsgfDisplay;
//---------------------------------------------------------
//		Constants
//---------------------------------------------------------
#[derive(PartialEq, Clone, Copy)]
pub enum NoteStatus {
    DuringNoteOn,
    AfterNoteOff,
    DuringDamp,
}
const DAMP_TIME: u32 = 300;		// * dac time(22.68usec)

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub trait Voice {
    fn start_sound(&mut self);
    fn slide(&mut self, _note: u8, _vel: u8){}
    fn note_off(&mut self);
    fn damp(&mut self);
    fn change_pmd(&mut self, value: f32);
    fn amplitude(&mut self, volume: u8, expression: u8);
    fn pitch(&mut self, pitch:f32);