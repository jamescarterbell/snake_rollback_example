use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rollback::*;

#[derive(Clone)]
pub struct SnakeInput{
    inputs: Vec<HashMap<Action, ActionState>>
}

impl SnakeInput{
    pub fn new(num_players: usize) -> Self{
        let mut inputs = Vec::new();
        
        for player in 0..num_players{
            inputs.push(Self::new_map());
        }

        Self{
            inputs,
        }
    }

    pub fn new_map() -> HashMap<Action, ActionState>{
        let mut map = HashMap::new();
        map.insert(Action::Up, ActionState::Up);
        map.insert(Action::Down, ActionState::Up);
        map.insert(Action::Left, ActionState::Up);
        map.insert(Action::Right, ActionState::Up);
        map.insert(Action::Boost, ActionState::Up);
        map
    }

    pub fn pressed(&self, player: usize, action: &Action) -> bool{
        self.inputs[player][action] == ActionState::Pressed
    }

    pub fn press(&mut self, player: usize, action: Action){
        self.inputs[player].insert(action, ActionState::Pressed);
    }

    pub fn released(&self, player: usize, action: &Action) -> bool{
        self.inputs[player][action] == ActionState::Released
    }
    
    pub fn release(&mut self, player: usize, action: Action){
        self.inputs[player].insert(action, ActionState::Released);
    }

    pub fn up(&self, player: usize, action: &Action) -> bool{
        self.inputs[player][action] == ActionState::Released ||
        self.inputs[player][action] == ActionState::Up 
    }

    pub fn set_up(&mut self, player: usize, action: Action){
        self.inputs[player].insert(action, ActionState::Up);
    }

    pub fn down(&self, player: usize, action: &Action) -> bool{
        self.inputs[player][action] == ActionState::Pressed ||
        self.inputs[player][action] == ActionState::Down
    }

    pub fn set_down(&mut self, player: usize, action: Action){
        self.inputs[player].insert(action, ActionState::Down);
    }
}

pub fn read_input(mut map: ResMut<SnakeInput>, inputs: Res<Input<KeyCode>>){
    let input = map.inputs.get_mut(0).unwrap();
    for (action, state) in input.iter_mut(){
        let key = match action{
            Action::Up => KeyCode::Up,
            Action::Down => KeyCode::Down,
            Action::Left => KeyCode::Left,
            Action::Right => KeyCode::Right,
            Action::Boost => KeyCode::Space,
        };

        match state{
            ActionState::Up if inputs.just_pressed(key) => *state = ActionState::Pressed,
            ActionState::Down if inputs.just_released(key) => *state = ActionState::Released,
            _ => {},
        };
    }
}

pub fn update_input_buffer(mut map: ResMut<SnakeInput>){
    for input in map.inputs.iter_mut(){
        for (action, state) in input.iter_mut(){
            *state = match state{
                ActionState::Up => ActionState::Up,
                ActionState::Down => ActionState::Down,
                ActionState::Pressed => ActionState::Down,
                ActionState::Released => ActionState::Up,
            };
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Action{
    Up,
    Down,
    Left,
    Right,
    Boost
}

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Debug)]
pub enum ActionState{
    Up = 0,
    Down,
    Released,
    Pressed,
}

pub struct SnakeInputPlugin;

impl Plugin for SnakeInputPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_logic_system_to_stage(logic_stages::LOGIC_POSTUPDATE, update_input_buffer.system())
            .add_system(read_input.system())
            .add_resource(SnakeInput::new(1))
            .override_resource::<SnakeInput>();
    }
}