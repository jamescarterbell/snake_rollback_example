use bevy_rollback::*;
use bevy::prelude::*;
use super::snake_visuals::*;
use super::snake_input::*;

pub struct SnakeLogic;

impl Plugin for SnakeLogic{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_system(spawn_snake.system())
            .add_logic_system(change_move_direction.system())
            .add_logic_system(move_snake.system())
            .add_system(test_rollback.system());
    }
}

pub struct SnakeHead{
    speed: i8,
    segments: Vec<Entity>,
    segments_added: i8,
}

pub struct Player{
    id: usize,
}

pub struct MoveDirection{
    direction: Vec2,
    boosted: bool,
    timer: i8,
    frame: u128,
}

fn spawn_snake(
    commands: &mut Commands,
    sprites: Res<SnakeSpriteHandles>,
){
    commands
        .spawn(SpriteSheetBundle{
            sprite: TextureAtlasSprite{
                index: sprites.sprites[&SnakeSprites::RedHead],
                .. Default::default()
            },
            texture_atlas: sprites.texture.clone(),
            .. Default::default()
        })
        .with(SnakeHead{
            speed: 1,
            segments: Vec::new(),
            segments_added: 0,
        })
        .with(Player{id: 0})
        .with(MoveDirection{
            direction: Vec2::zero(),
            boosted: false,
            timer: 1,
            frame: 0,
        })
        .with(RollbackTracked);
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
        println!("Frame: {}", dir.frame);
        dir.frame += 1;
        if dir.timer <= 0{
            dir.timer = head.speed;
            let mut last_position = transform.translation;
            transform.translation += Vec3::from((dir.direction * 16.0, 0.0));

            for segment in head.segments.iter(){
                
            }
        }
        else{
            dir.timer -= 1;
        }
    }

}

fn test_rollback(
    input: Res<SnakeInput>,
    mut rollback_buffer: ResMut<RollbackBuffer>,
){
    if input.pressed(0, &Action::Boost){
        println!("CHANGING PAST!");
        let frame = rollback_buffer.newest_frame - 20;
        rollback_buffer.past_frame_change(
            frame, 
            Box::new(|mut input: ResMut<SnakeInput>| {
                println!("IN THE PAST!");
                input.press(0, Action::Up);
            }).system()
        );
    }
}