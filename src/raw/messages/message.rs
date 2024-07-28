use crate::raw::messages::clutter_filter_map;
use crate::raw::messages::digital_radar_data;
use crate::raw::messages::message_header::MessageHeader;
use crate::raw::messages::rda_status_data;

/// A decoded NEXRAD Level II message with its metadata header.
#[derive(Debug)]
pub struct MessageWithHeader {
    pub header: MessageHeader,
    pub message: Message,
}

/// A decoded NEXRAD Level II message.
#[derive(Debug)]
pub enum Message {
    RDAStatusData(Box<rda_status_data::Message>),
    DigitalRadarData(Box<digital_radar_data::Message>),
    ClutterFilterMap(Box<clutter_filter_map::Message>),
    Other,
}
