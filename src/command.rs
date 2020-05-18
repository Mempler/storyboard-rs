use super::CommandGroup;

use cgmath::{Vector2, Vector4};

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum CommandType {
    None,
    Movement,
    MovementX,
    MovementY,
    Fade,
    Scale,
    VectorScale,
    Rotation,
    Colour,
    FlipHorizontal,
    FlipVertical,
    BlendingMode,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Easing {
    None,
    Out,
    In,
    InQuad,
    OutQuad,
    InOutQuad,
    InCubic,
    OutCubic,
    InOutCubic,
    InQuart,
    OutQuart,
    InOutQuart,
    InQuint,
    OutQuint,
    InOutQuint,
    InSine,
    OutSine,
    InOutSine,
    InExpo,
    OutExpo,
    InOutExpo,
    InCirc,
    OutCirc,
    InOutCirc,
    InElastic,
    OutElastic,
    OutElasticHalf,
    OutElasticQuarter,
    InOutElastic,
    InBack,
    OutBack,
    InOutBack,
    InBounce,
    OutBounce,
    InOutBounce,
}

#[derive(Debug, Copy, Clone)]
pub struct Command {
    pub ctype: CommandType,
    pub easing: Easing,
    pub start_time: i32,
    pub end_time: i32,
    pub start_colour: Vector4<f32>,
    pub end_colour: Vector4<f32>,
    pub start_vector: Vector2<f32>,
    pub end_vector: Vector2<f32>,
    pub start_value: f32,
    pub end_value: f32,
}

impl Command {
    pub fn from_string(data: String) -> Command {
        let mut columns: Vec<&str> = data.split(",").collect();

        if columns.len() < 4 {
            columns.push("")
        }
        if columns[3] == "" {
            columns[3] = columns[2]
        }

        let easing = match columns[1] {
            "1" => Easing::Out,
            "2" => Easing::In,
            "3" => Easing::InQuad,
            "4" => Easing::OutQuad,
            "5" => Easing::InOutQuad,
            "6" => Easing::InCubic,
            "7" => Easing::OutCubic,
            "8" => Easing::InOutCubic,
            "9" => Easing::InQuart,
            "10" => Easing::OutQuart,
            "11" => Easing::InOutQuart,
            "12" => Easing::InQuint,
            "13" => Easing::OutQuint,
            "14" => Easing::InOutQuint,
            "15" => Easing::InSine,
            "16" => Easing::OutSine,
            "17" => Easing::InOutSine,
            "18" => Easing::InExpo,
            "19" => Easing::OutExpo,
            "20" => Easing::InOutExpo,
            "21" => Easing::InCirc,
            "22" => Easing::OutCirc,
            "23" => Easing::InOutCirc,
            "24" => Easing::InElastic,
            "25" => Easing::OutElastic,
            "26" => Easing::OutElasticHalf,
            "27" => Easing::OutElasticQuarter,
            "28" => Easing::InOutElastic,
            "29" => Easing::InBack,
            "30" => Easing::OutBack,
            "31" => Easing::InOutBack,
            "32" => Easing::InBounce,
            "33" => Easing::OutBounce,
            "34" => Easing::InOutBounce,

            _ => Easing::None,
        };

        let start_time = columns[2].parse::<i32>().unwrap();
        let end_time = columns[3].parse::<i32>().unwrap();

        match columns[0] {
            "F" => {
                let start_value = columns[4].parse::<f32>().unwrap();
                let mut end_value = 0.0f32;
                if columns.len() > 5 {
                    end_value = columns[4].parse::<f32>().unwrap();
                }
                Command {
                    ctype: CommandType::Fade,
                    easing,
                    start_time,
                    end_time,
                    start_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    end_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    start_vector: Vector2::<f32>::new(0.0, 0.0),
                    end_vector: Vector2::<f32>::new(0.0, 0.0),
                    start_value,
                    end_value,
                }
            }
            "M" => {
                let start_x = columns[4].parse::<f32>().unwrap();
                let start_y = columns[5].parse::<f32>().unwrap();

                let mut end_x = start_x;
                let mut end_y = start_y;
                if columns.len() > 6 {
                    end_x = columns[6].parse::<f32>().unwrap();
                }

                if columns.len() > 7 {
                    end_y = columns[7].parse::<f32>().unwrap();
                }

                Command {
                    ctype: CommandType::Movement,
                    easing,
                    start_time,
                    end_time,
                    start_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    end_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    start_vector: Vector2::<f32>::new(start_x, start_y),
                    end_vector: Vector2::<f32>::new(end_x, end_y),
                    start_value: 0.0,
                    end_value: 0.0,
                }
            }
            "MX" => {
                let start_x = columns[4].parse::<f32>().unwrap();

                let mut end_x = start_x;
                if columns.len() > 5 {
                    end_x = columns[5].parse::<f32>().unwrap();
                }

                Command {
                    ctype: CommandType::MovementX,
                    easing,
                    start_time,
                    end_time,
                    start_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    end_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    start_vector: Vector2::<f32>::new(start_x, 0.0),
                    end_vector: Vector2::<f32>::new(end_x, 0.0),
                    start_value: 0.0,
                    end_value: 0.0,
                }
            }
            "MY" => {
                let start_y = columns[4].parse::<f32>().unwrap();

                let mut end_y = start_y;

                if columns.len() > 5 {
                    end_y = columns[5].parse::<f32>().unwrap();
                }

                Command {
                    ctype: CommandType::MovementY,
                    easing,
                    start_time,
                    end_time,
                    start_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    end_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    start_vector: Vector2::<f32>::new(0.0, start_y),
                    end_vector: Vector2::<f32>::new(0.0, end_y),
                    start_value: 0.0,
                    end_value: 0.0,
                }
            }
            "S" => {
                let start_value = columns[4].parse::<f32>().unwrap();
                let mut end_value = 0.0f32;
                if columns.len() > 5 {
                    end_value = columns[4].parse::<f32>().unwrap();
                }
                Command {
                    ctype: CommandType::Scale,
                    easing,
                    start_time,
                    end_time,
                    start_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    end_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    start_vector: Vector2::<f32>::new(0.0, 0.0),
                    end_vector: Vector2::<f32>::new(0.0, 0.0),
                    start_value,
                    end_value,
                }
            }
            "V" => {
                let start_x = columns[4].parse::<f32>().unwrap();
                let start_y = columns[5].parse::<f32>().unwrap();

                let mut end_x = start_x;
                let mut end_y = start_y;
                if columns.len() > 6 {
                    end_x = columns[6].parse::<f32>().unwrap();
                }

                if columns.len() > 7 {
                    end_y = columns[7].parse::<f32>().unwrap();
                }

                Command {
                    ctype: CommandType::VectorScale,
                    easing,
                    start_time,
                    end_time,
                    start_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    end_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    start_vector: Vector2::<f32>::new(start_x, start_y),
                    end_vector: Vector2::<f32>::new(end_x, end_y),
                    start_value: 0.0,
                    end_value: 0.0,
                }
            }
            "R" => {
                let start_value = columns[4].parse::<f32>().unwrap();
                let mut end_value = 0.0f32;
                if columns.len() > 5 {
                    end_value = columns[4].parse::<f32>().unwrap();
                }
                Command {
                    ctype: CommandType::Rotation,
                    easing,
                    start_time,
                    end_time,
                    start_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    end_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                    start_vector: Vector2::<f32>::new(0.0, 0.0),
                    end_vector: Vector2::<f32>::new(0.0, 0.0),
                    start_value,
                    end_value,
                }
            }
            "C" => {
                let start_r = columns[4].parse::<f32>().unwrap();
                let start_g = columns[5].parse::<f32>().unwrap();
                let start_b = columns[6].parse::<f32>().unwrap();

                let mut end_r = 0.0f32;
                let mut end_g = 0.0f32;
                let mut end_b = 0.0f32;

                if columns.len() > 7 {
                    end_r = columns[7].parse::<f32>().unwrap();
                }
                if columns.len() > 8 {
                    end_g = columns[8].parse::<f32>().unwrap();
                }
                if columns.len() > 9 {
                    end_b = columns[9].parse::<f32>().unwrap();
                }

                Command {
                    ctype: CommandType::VectorScale,
                    easing,
                    start_time,
                    end_time,
                    start_colour: Vector4::<f32>::new(start_r, start_g, start_b, 1.0),
                    end_colour: Vector4::<f32>::new(end_r, end_g, end_b, 1.0),
                    start_vector: Vector2::<f32>::new(0.0, 0.0),
                    end_vector: Vector2::<f32>::new(0.0, 0.0),
                    start_value: 0.0,
                    end_value: 0.0,
                }
            }
            "P" => Command {
                ctype: match columns[4] {
                    "H" => CommandType::FlipHorizontal,
                    "V" => CommandType::FlipVertical,
                    "A" => CommandType::BlendingMode,

                    _ => unimplemented!("Unimplemented Command Type {}", columns[4]),
                },
                easing,
                start_time,
                end_time,
                start_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 1.0),
                end_colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 1.0),
                start_vector: Vector2::<f32>::new(0.0, 0.0),
                end_vector: Vector2::<f32>::new(0.0, 0.0),
                start_value: 0.0,
                end_value: 0.0,
            },

            _ => unimplemented!("Unknown CommandType: {}", columns[0]),
        }
    }
}

#[derive(Debug)]
pub struct LoopCommand {
    pub start_time: i32,
    pub count: i32,

    pub commands: CommandGroup,
}

impl LoopCommand {
    pub fn new(start_time: i32, count: i32) -> LoopCommand {
        LoopCommand {
            start_time,
            count,

            commands: CommandGroup::new(),
        }
    }
}

#[derive(Debug)]
pub struct TriggerCommand {
    pub name: String,
    pub start_time: i32,
    pub end_time: i32,
    pub group_number: i32,
    pub commands: CommandGroup,
}

impl TriggerCommand {
    pub fn new(name: String, start_time: i32, end_time: i32, group_number: i32) -> TriggerCommand {
        TriggerCommand {
            name,
            start_time,
            end_time,
            group_number,

            commands: CommandGroup::new(),
        }
    }
}
