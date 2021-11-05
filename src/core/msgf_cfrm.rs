//
//  msgf_cfrm.rs
//	Musical Sound Generator Framework
//      Control Frame Class
//
//  Created by Hasebe Masahiko on 2021/10/24.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct CtrlFrame {
    cbuf: Vec<f32>,
    pub sample_number: usize,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl CtrlFram