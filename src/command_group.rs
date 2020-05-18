use super::{Command, LoopCommand, TriggerCommand};

#[derive(Debug)]
pub struct CommandGroup {
    pub commands: Vec<Command>,
    pub loop_commands: Vec<LoopCommand>,
    pub trigger_commands: Vec<TriggerCommand>,
}

impl CommandGroup {
    pub fn new() -> CommandGroup {
        CommandGroup {
            commands: Vec::<Command>::new(),
            loop_commands: Vec::<LoopCommand>::new(),
            trigger_commands: Vec::<TriggerCommand>::new(),
        }
    }

    pub fn add_trigger(&mut self, command: TriggerCommand) -> &mut CommandGroup {
        self.trigger_commands.push(command);

        &mut self.trigger_commands.last_mut().unwrap().commands
    }

    pub fn add_loop(&mut self, command: LoopCommand) -> &mut CommandGroup {
        self.loop_commands.push(command);

        &mut self.loop_commands.last_mut().unwrap().commands
    }
}
