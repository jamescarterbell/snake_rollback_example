use std::collections::HashSet;

use bevy_rollback::*;
use bevy::prelude::*;
use super::snake_visuals::*;
use super::snake_input::*;
use rand::{rngs::ThreadRng, thread_rng};
use rand::Rng;

pub struct SnakeLogic;

impl Plugin for SnakeLogic{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_system(spawn_snake.system())
            .add_logic_system(change_move_direction.system())
            .add_logic_system(move_snake.system())
            .add_logic_system(food_spawner.system())
            .track_resource(FoodCounter{
                food_count: 0,
                current_food: 0,
                food_positions: HashSet::new()
            })
            .track_resource(Rand{
                rand: thread_rng(),
            });
    }
}

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct SnakeHead{
    speed: i8,
    segments: Vec<Entity>,
    segments_added: i8,
}

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Player{
    id: usize,
}

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct MoveDirection{
    pub direction: Vec2,
    boosted: bool,
    timer: i8,
    frame: u128,
    boost: i8,
}

fn spawn_snake(
    commands: &mut Commands,
    rollback_buffer: Res<RollbackBuffer>,
    sprites: Res<SnakeSpriteHandles>,
){
    let mut logic_commands = rollback_buffer.get_logic_commands_builder();
    logic_commands
        .commands
        .spawn((
            Transform{
                translation: Vec3::zero(),
                .. Default::default()
            },
            SnakeHead{
                speed: 10,
                segments: Vec::new(),
                segments_added: 0,
            },
            Player{id: 0},
            MoveDirection{
            direction: Vec2::zero(),
            boosted: false,
            timer: 1,
            frame: 0,
            boost: 10,
        }));

    {
        let mut registry = rollback_buffer.logic_registry.write();
        registry.register::<SnakeHead>();
        registry.register::<Player>();
        registry.register::<MoveDirection>();
        registry.register::<Transform>();
        registry.register::<Food>();
        registry.register::<DoubleFood>();
    }

    commands.add_command(logic_commands.build());
}

fn change_move_direction(
    input: Res<SnakeInput>,
    mut character: Query<(&mut MoveDirection, &Player)>,
){
    for (mut dir, player) in character.iter_mut(){
        if input.down(player.id, &Action::Boost){
            dir.boosted = true;
        }

        if input.pressed(player.id, &Action::Up) && dir.direction != Vec2::new(0.0, -1.0){
            dir.direction = Vec2::new(0.0, 1.0);
        }
        if input.pressed(player.id, &Action::Down) && dir.direction != Vec2::new(0.0, 1.0){
            dir.direction = Vec2::new(0.0, -1.0);
        }
        if input.pressed(player.id, &Action::Left) && dir.direction != Vec2::new(1.0, 0.0){
            dir.direction = Vec2::new(-1.0, 0.0);
        }
        if input.pressed(player.id, &Action::Right) && dir.direction != Vec2::new(-1.0, 0.0){
            dir.direction = Vec2::new(1.0, 0.0);
        }
    }
}

fn move_snake(
    mut character: Query<(&mut Transform, &mut MoveDirection, &mut SnakeHead)>,
){
    for (mut transform, mut dir, head) in character.iter_mut(){
        dir.frame += 1;
        if dir.timer <= 0{
            dir.timer = head.speed;
            let mut last_position = transform.translation;
            transform.translation += Vec3::from((dir.direction * 16.0, 0.0));
        }
        else{
            if dir.boost == 0 && dir.boosted{
                dir.boosted = false;
            }
            dir.timer -= if dir.boosted {2} else {1};
            dir.boost -= if dir.boosted {1} else {0};
        }
    }
}

#[derive(Clone)]
struct Rand{
    rand: ThreadRng,
}

unsafe impl Send for Rand{}
unsafe impl Sync for Rand{}


#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Food;

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct DoubleFood;

#[derive(Clone)]
pub struct FoodCounter{
    food_count: u32,
    current_food: u8,
    food_positions: HashSet<(i32, i32)>,
}

fn food_spawner(
    commands: &mut Commands,
    mut current_food: ResMut<FoodCounter>,
    mut rand: ResMut<Rand>,
){
    while current_food.current_food < std::cmp::max((current_food.food_count as f64).log2() as u8, 1){
        let pos = loop{
            let pos = Vec2::new(rand.rand.gen_range(-20..20) as f32, rand.rand.gen_range(-20..20) as f32);
            if !current_food.food_positions.contains(&(pos.x as i32, pos.y as i32)){
                current_food.food_positions.insert((pos.x as i32, pos.y as i32));
                break pos;
            }
        };

        if 1 == rand.rand.gen::<u8>() & 1{
            commands.spawn((
                Food,
                Transform{
                    translation: Vec3::from((pos, 0.0)),
                    .. Default::default()
                }));
        }
        else{
            commands.spawn((
                Food,
                Transform{
                    translation: Vec3::from((pos, 0.0)),
                    .. Default::default()
                }));
        }
        println!("SUCKING PP");
        current_food.current_food += 1;
    }
}