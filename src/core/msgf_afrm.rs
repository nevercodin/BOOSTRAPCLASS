
//
//  msgf_afrm.rs
//	Musical Sound Generator Framework
//      Audio Frame Class
//
//  Created by Hasebe Masahiko on 2021/09/24.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct AudioFrame {
    abuf: Vec<f32>,