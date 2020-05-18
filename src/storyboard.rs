use super::{Command, CommandGroup, Event, EventType, LoopCommand, TriggerCommand};

#[derive(Debug)]
pub struct Storyboard {
    pub events: Vec<Event>,
}

impl Storyboard {
    pub fn new() -> Storyboard {
        Storyboard {
            events: Vec::<Event>::new(),
        }
    }

    pub fn from_string(data: String) -> Storyboard {
        let mut storyboard = Storyboard::new();
        let mut root_group = CommandGroup::new();
        let mut command_group = &mut root_group;

        let mut last_event = &Event::new();
        let events = data.split("\r\n");
        for event in events {
            if event.starts_with("//") || event.starts_with("[Events]") || event.trim().len() <= 0 {
                continue;
            }

            if event.to_string().starts_with(" ") {
                let mut cmd_raw = event;
                let mut depth = 0;

                while cmd_raw.starts_with(" ") {
                    depth += 1;
                    cmd_raw = &cmd_raw[1..];
                }

                if depth < 2 {
                    if last_event.etype == EventType::Unknown {
                        continue;
                    }

                    command_group = &mut root_group;
                }

                let columns: Vec<&str> = cmd_raw.split(",").collect();
                match columns[0] {
                    "T" => {
                        let name = columns[1];
                        let mut start_time = 0;
                        let mut end_time = 0;
                        let mut group_number = 0;

                        if columns.len() > 2 {
                            start_time = columns[2].parse::<i32>().unwrap();
                        }
                        if columns.len() > 3 {
                            end_time = columns[3].parse::<i32>().unwrap();
                        }
                        if columns.len() > 4 {
                            group_number = columns[4].parse::<i32>().unwrap();
                        }

                        command_group = command_group.add_trigger(TriggerCommand::new(
                            name.to_string(),
                            start_time,
                            end_time,
                            group_number,
                        ));
                    }

                    "L" => {
                        let start_time = columns[1].parse::<i32>().unwrap();
                        let count = columns[2].parse::<i32>().unwrap();
                        command_group = command_group.add_loop(LoopCommand::new(start_time, count));
                    }
                    _ => command_group
                        .commands
                        .push(Command::from_string(cmd_raw.to_string())),
                }

                continue;
            }

            let mut ev = Event::from_string(event.to_string());
            ev.commands = root_group.commands.to_owned();
            storyboard.events.push(ev);
            last_event = storyboard.events.last_mut().unwrap();
            root_group = CommandGroup::new();
            command_group = &mut root_group;
        }

        storyboard
    }
}
