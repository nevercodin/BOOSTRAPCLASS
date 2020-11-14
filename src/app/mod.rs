//
//  app/mod.rs
//	Musical Sound Generator Framework
//
//  Created by Hasebe Masahiko on 2021/10/25.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::msgf_inst;

//  You can select a specific app.
pub mod va;
pub mod sg;
pub mod sgf;
use crate::app::va::*;
use crate::app::sg::*;
use crate::app::sgf::*;
pub fn get_in