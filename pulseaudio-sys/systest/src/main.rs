#![allow(bad_style)]

extern crate pulseaudio_sys;

use pulseaudio_sys::*;
use std::os::raw::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
