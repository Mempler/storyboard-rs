use super::Command;

use cgmath::{Vector2};

#[derive(PartialEq, Debug)]
pub enum EventType {
    Sprite,
    Animation,
    Sample,
    Unknown,
}

#[derive(Debug)]
pub enum LayerType {
    Background,
    Fail,
    Pass,
    Foreground,
    Overlay,
    Samples,

    Unknown,
}

#[derive(Debug)]
pub enum OriginType {
    TopLeft,
    TopCentre,
    TopRight,
    CentreLeft,
    Centre,
    CentreRight,
    BottomLeft,
    BottomCentre,
    BottomRight,

    Custom,
}

#[derive(Debug)]
pub enum LoopType {
    LoopForever,
    LoopOnce,
}

#[derive(Debug)]
pub struct Event {
    pub etype: EventType,
    pub layer: LayerType,
    /* Event: Shared */
    pub file_path: String,
    pub pos: Vector2<f32>,
    pub origin: OriginType,
    pub commands: Vec<Command>,

    /* Event: Animation */
    pub frame_count: i32,
    pub frame_delay: f64,
    pub loop_type: LoopType,

    /* Event: Sample */
    pub time: i32,
    pub volume: i32,
}

impl Event {
    pub fn new() -> Event {
        Event {
            etype: EventType::Unknown,
            layer: LayerType::Unknown,
            file_path: String::default(),
            pos: Vector2::<f32>::new(0.0, 0.0),
            origin: OriginType::TopLeft,
            commands: Vec::<Command>::new(),
            frame_count: 0,
            frame_delay: 0.0,
            loop_type: LoopType::LoopOnce,
            time: 0,
            volume: 0,
        }
    }

    pub fn from_string(data: String) -> Event {
        let columns: Vec<&str> = data.split(",").collect();

        let event_type = match columns[0] {
            "Sprite" => EventType::Sprite,
            "Animation" => EventType::Animation,
            "Sample" => EventType::Sample,

            _ => EventType::Unknown,
        };

        let event_layer = match columns[1] {
            "Background" => LayerType::Background,
            "Fail" => LayerType::Fail,
            "Pass" => LayerType::Pass,
            "Foreground" => LayerType::Foreground,
            "Overlay" => LayerType::Overlay,
            "Samples" => LayerType::Samples,

            _ => LayerType::Unknown,
        };

        let event_origin = match columns[2] {
            "TopLeft" => OriginType::TopLeft,
            "TopCentre" => OriginType::TopCentre,
            "TopRight" => OriginType::TopRight,
            "CentreLeft" => OriginType::CentreLeft,
            "Centre" => OriginType::Centre,
            "CentreRight" => OriginType::CentreRight,
            "BottomLeft" => OriginType::BottomLeft,
            "BottomCentre" => OriginType::BottomCentre,
            "BottomRight" => OriginType::BottomRight,

            _ => OriginType::Custom,
        };

        let file_path = columns[3].trim_matches('"');

        let position_x = columns[4].parse::<f32>().unwrap(); // TODO: test for errors
        let position_y = columns[5].parse::<f32>().unwrap(); // TODO: here to.

        let position = Vector2::<f32>::new(position_x, position_y);
        match event_type {
            EventType::Sprite => {
                return Event {
                    etype: event_type,
                    layer: event_layer,

                    /* Event: Shared */
                    file_path: file_path.to_string(),
                    pos: position,
                    origin: event_origin,
                    commands: Vec::<Command>::new(),
                    /* Event: Animation */
                    frame_count: 0,
                    frame_delay: 0.0,
                    loop_type: LoopType::LoopOnce,
                    /* Event: Sample */
                    time: 0,
                    volume: 0,
                };
            }
            EventType::Animation => {
                let mut loop_type = LoopType::LoopForever;
                if columns.len() > 8 {
                    loop_type = match columns[8] {
                        "LoopOnce" => LoopType::LoopOnce,

                        _ => LoopType::LoopForever,
                    }
                }

                return Event {
                    etype: event_type,
                    layer: event_layer,
                    file_path: file_path.to_string(),
                    pos: position,
                    origin: event_origin,
                    commands: Vec::<Command>::new(),

                    /* Event: Animation */
                    frame_count: columns[6].parse::<i32>().unwrap(),
                    frame_delay: columns[7].parse::<f64>().unwrap(),
                    loop_type: loop_type,
                    /* Event: Sample */
                    time: 0,
                    volume: 0,
                };
            }

            _ => println!("Unimplemented Storyboard Event! {:#?}", event_type),
        };

        Event::new() // Return empty event if not implemented
    }
}
