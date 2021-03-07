
//
//  sg_voice.rs
//	Musical Sound Generator Framework
//      Sing Voice Class
//
//  Created by Hasebe Masahiko on 2022/03/15.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use std::rc::Rc;
use std::cell::Cell;
use crate::msgf_if;
use crate::core::*;
use crate::core::msgf_voice::*;
use crate::core::msgf_disp::MsgfDisplay;
use crate::engine::*;
use crate::engine::msgf_gen::Engine;
use crate::app::sg::*;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct VoiceSg {
    // Note
    note: u8,
    vel: u8,
    status: NoteStatus,
    damp_counter: u32,