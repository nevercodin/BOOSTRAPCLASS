
//
//  msgf_if.rs
//	Musical Sound Generator Framework
//      Interface for C / Objective-C
//
//  Created by Hasebe Masahiko on 2021/09/12.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::*;
use crate::engine::*;
use crate::engine::msgf_gen::*;
use crate::core::msgf_disp::MsgfDisplay;
//---------------------------------------------------------
//		Constants
//---------------------------------------------------------
//  configuration
pub const REV_NUM: &str = "rev.0-0-9";
pub const MAX_PART_NUM: usize = 10;
pub const MAX_BUFFER_SIZE: usize = 1024;
pub const SAMPLING_FREQ: f32 = 44100.0;
pub const PI: f32 = std::f32::consts::PI;
pub const AUDIO_FRAME_PER_CONTROL: usize = 128;
pub const DAMP_LIMIT_DEPTH: f32 = 0.0001;
pub const TOTAL_EFF_DLY_TIME_L: f32 = 0.25;
pub const TOTAL_EFF_DLY_TIME_R: f32 = 0.27;
pub const TOTAL_EFF_ATT_RATE: f32 = 0.3;
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Msgf {
    msg_buf: Vec<(u8,usize,u8,u8)>,
    part: Vec<msgf_part::Part>,
    audio_buffer_l: msgf_afrm::AudioFrame,
    audio_buffer_r: msgf_afrm::AudioFrame,
    audio_buffer_send_effect_l: msgf_afrm::AudioFrame,
    audio_buffer_send_effect_r: msgf_afrm::AudioFrame,
    audio_buffer_total_effect_l: msgf_afrm::AudioFrame,
    audio_buffer_total_effect_r: msgf_afrm::AudioFrame,
    delay: msgf_sd_delay::SdDelay,
    in_number_frames: u32,
}