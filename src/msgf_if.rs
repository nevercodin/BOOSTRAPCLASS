
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
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl msgf_disp::MsgfDisplay for Msgf {}
impl Msgf {
    pub fn new() -> Self {
        let dprm = msgf_delay::DelayParameter {
            l_time: TOTAL_EFF_DLY_TIME_L,   //  0.0 - 1.0 [sec]
            r_time: TOTAL_EFF_DLY_TIME_R,   //  0.0 - 1.0 [sec]
            att_ratio: TOTAL_EFF_ATT_RATE,
        };        
        Self {
            msg_buf: Vec::new(),
            part: Vec::new(),
            audio_buffer_l: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_r: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_send_effect_l: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_send_effect_r: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_total_effect_l: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_total_effect_r: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            delay: msgf_sd_delay::SdDelay::new(&dprm),
            in_number_frames: 0,
        }
    }
    pub fn init(&mut self) {    // call this fn just after new()
        for _ in 0..MAX_PART_NUM {
            self.part.push(msgf_part::Part::new());
        };
        self.print_str(REV_NUM);
    }
    pub fn recieve_midi_message(&mut self, dt1: u8, dt2: u8, dt3: u8) {
        let ch: usize = (dt1 & 0x0f).into();
        let status = dt1 & 0xf0;

        if ch >= MAX_PART_NUM {
            return;
        };