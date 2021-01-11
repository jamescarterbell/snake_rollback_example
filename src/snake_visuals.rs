use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::collections::HashMap;

pub mod stage{
    pub const LOAD_ASSETS: &str = "load_assets";
}

pub struct SnakeSpriteHandles{
    pub texture: Handle<TextureAtlas>,
    pub sprites: HashMap<SnakeSprites, u32>,
}

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Debug)]
pub enum SnakeSprites{
    RedHead,
    RedStraight,
    RedBend,
    RedTail,
    BlueHead,
    BlueStraight,
    BlueBend,
    BlueTail,
    Cherry,
    Orange,
}

pub struct SnakeVisualsPlugin;

impl Plugin for SnakeVisualsPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_resource(ClearColor(Color::rgb(0.025, 0.5, 0.05)))
            .add_resource(WindowDescriptor { 
                title: "Snake!".to_string(), 
                width: 524.0,                 
                height: 524.0,      
                resizable: false,          
                ..Default::default()         
            })
            .add_startup_stage_before(
                bevy::app::stage::STARTUP,
                stage::LOAD_ASSETS,
                SystemStage::parallel(),
            )
            .add_startup_system_to_stage(stage::LOAD_ASSETS, setup.system());
    }
}

fn setup(
    commands: &mut Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
){

    let texture_handle = asset_server.load(r#"sprites.png"#);

    let mut texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 4);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .insert_resource(
            SnakeSpriteHandles{
                texture: texture_atlas_handle.clone(),
                sprites: {
                    let mut map = HashMap::new();
                    map.insert(SnakeSprites::RedHead, 0);
                    map.insert(SnakeSprites::BlueHead, 1);
                    map.insert(SnakeSprites::Cherry, 2);
                    map.insert(SnakeSprites::Orange, 3);
                    map.insert(SnakeSprites::RedStraight, 4);
                    map.insert(SnakeSprites::BlueStraight, 5);
                    map.insert(SnakeSprites::RedBend, 8);
                    map.insert(SnakeSprites::BlueBend, 9);
                    map.insert(SnakeSprites::RedTail, 12);
                    map.insert(SnakeSprites::BlueTail, 13);
                    map
                }
            }
        )
        .spawn(Camera2dBundle::default());    
}