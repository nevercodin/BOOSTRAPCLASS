//
//  va_inst.rs
//	Musical Sound Generator Framework
//      Virtual Analog Instrument Class
//
//  Created by Hasebe Masahiko on 2021/11/21.
//  Copyright (c) 2021 Hasebe Masahiko.
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
use crate::app::va::*;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
const MAX_PB_RANGE:f32 = 1200.0;
const MIDI_MAX_PB_VAL:f32 = 8192.0;
const MIDI_CENTER_VAL:f32 = 64.0;
//------------