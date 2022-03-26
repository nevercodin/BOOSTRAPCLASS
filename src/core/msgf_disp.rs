//
//  msgf_disp.rs
//	Musical Sound Generator Framework
//      Display
//
//  Created by Hasebe Masahiko on 2022/08/21.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub trait MsgfDisplay {
    fn print_str(&self, string: &str) {println!("{}",string)}
    fn print