// bleat_core Source Code File
// Copyright 2024 Alt-Innovations. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.
//
// Some portions of this file are taken and/or modified from btleplug
// (https://github.com/deviceplug/btleplug), BSD 3-Clause license under the
// following copyright:
// Copyright 2020 Nonpolynomial Labs LLC.

use crate::api::ValueNotification;
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;
use tokio::sync::broadcast::Receiver;
use tokio_stream::wrappers::BroadcastStream;

pub fn notifications_stream_from_broadcast_receiver(
    receiver: Receiver<ValueNotification>,
) -> Pin<Box<dyn Stream<Item = ValueNotification> + Send>> {
    Box::pin(BroadcastStream::new(receiver).filter_map(|x| async move { x.ok() }))
}
