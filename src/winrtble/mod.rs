// bleat_core Source Code File
//
// Copyright 2024 Alt-Innovations. All rights reserved.

//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.
//
// Some portions of this file are taken and/or modified from btleplug
// (https://github.com/deviceplug/btleplug), BSD 3-Clause license under the
// following copyright:
// Copyright 2020 Nonpolynomial Labs LLC.
//
// Some portions of this file are taken and/or modified from Rumble
// (https://github.com/mwylde/rumble), using a dual MIT/Apache License under the
// following copyright:
//
// Copyright (c) 2014 The Rust Project Developers

pub mod adapter;
mod ble;
pub mod manager;
pub mod peripheral;
mod utils;

/// Only some of the assigned numbers are populated here as needed from https://www.bluetooth.com/specifications/assigned-numbers/generic-access-profile/
mod advertisement_data_type {
    pub const SERVICE_DATA_16_BIT_UUID: u8 = 0x16;
    pub const SERVICE_DATA_32_BIT_UUID: u8 = 0x20;
    pub const SERVICE_DATA_128_BIT_UUID: u8 = 0x21;
}
