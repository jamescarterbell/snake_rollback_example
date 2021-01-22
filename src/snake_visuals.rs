use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::render::draw::{DrawContext, DrawError};
use bevy::render::pipeline::{PipelineSpecialization, PipelineDescriptor, VertexBufferDescriptor, InputStepMode, VertexAttributeDescriptor, VertexFormat };
use bevy::render::mesh;
use bevy::render::renderer::{BindGroup, RenderResourceBindings, RenderResourceId};
use std::collections::HashMap;
use bevy_rollback::LQuery;
use crate::{MoveDirection, SnakeHead, Player};
use bevy::sprite::{QUAD_HANDLE, SPRITE_PIPELINE_HANDLE, SPRITE_SHEET_PIPELINE_HANDLE};
use std::borrow::Cow;
use std::ops::Range;

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
            .add_startup_system_to_stage(stage::LOAD_ASSETS, setup.system())
            .add_system_to_stage(bevy::app::stage::POST_UPDATE, draw_snake.system());
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

fn draw_snake(
    mut context: DrawContext,
    characters: LQuery<(&Transform, &SnakeHead, &Player, &MoveDirection)>,
    sprite_handles: Res<SnakeSpriteHandles>,
    texture_atlases: Res<Assets<TextureAtlas>>){

    let mut drawer = SpriteAtlasDrawer::new(&mut context, 1);

    for (transform, head, player, dir) in characters.iter(){
        drawer.draw_sprite_atlas(&transform, &sprite_handles.texture, *sprite_handles.sprites.get(&SnakeSprites::RedHead).unwrap(), Color::rgb(1.0, 1.0, 1.0));
    }
}

struct SpriteAtlasDrawer<'a, 'b>{
    draw: Draw,
    draw_context: &'b mut DrawContext<'a>
}

impl<'b, 'a> SpriteAtlasDrawer<'a, 'b>{
    pub fn new(draw_context: &'b mut DrawContext<'a>, msaa_samples: u32) -> Self{
        let mut draw = Draw::default();
        draw_context.set_pipeline(
            &mut draw,
            &SPRITE_SHEET_PIPELINE_HANDLE.typed(),
            &PipelineSpecialization{
                sample_count: msaa_samples,
                vertex_buffer_descriptor: VertexBufferDescriptor{
                    name: Cow::Borrowed("sprite_texture"),
                    stride: std::mem::size_of::<f32>() as u64 * 8,
                    step_mode: InputStepMode::Vertex,
                    attributes: vec![
                        VertexAttributeDescriptor{
                            name: Cow::Borrowed("Vertex_Position"),
                            offset: 0,
                            shader_location: 0,
                            format: VertexFormat::Float3
                        },
                        VertexAttributeDescriptor{
                            name: Cow::Borrowed("Vertex_Normal"),
                            offset: std::mem::size_of::<f32>() as u64 * 3,
                            shader_location: 1,
                            format: VertexFormat::Float3
                        },
                        VertexAttributeDescriptor{
                            name: Cow::Borrowed("Vertex_Uv"),
                            offset: std::mem::size_of::<f32>() as u64 * 2,
                            shader_location: 2,
                            format: VertexFormat::Float3
                        },
                    ]
                },
                ..Default::default()
            }
        );

        Self{
            draw_context,
            draw,
        }
    }

    fn set_mesh_attributes(&mut self) -> Result<Range<u32>, DrawError>{
        let render_resource_context = &**self.draw_context.render_resource_context;

        if let Some(RenderResourceId::Buffer(vertex_attribute_buffer_id)) = render_resource_context
            .get_asset_resource(
                &QUAD_HANDLE.typed::<Mesh>(),
                mesh::VERTEX_ATTRIBUTE_BUFFER_ID,
            )
        {
            self.draw.set_vertex_buffer(0, vertex_attribute_buffer_id, 0);
        } else {
            println!("Could not find vertex buffer for `bevy_sprite::QUAD_HANDLE`.")
        }

        let mut indices = 0..0;
        if let Some(RenderResourceId::Buffer(quad_index_buffer)) = render_resource_context
            .get_asset_resource(
                &QUAD_HANDLE.typed::<Mesh>(),
                mesh::INDEX_BUFFER_ASSET_INDEX,
            )
        {
            self.draw.set_index_buffer(quad_index_buffer, 0);
            if let Some(buffer_info) = render_resource_context.get_buffer_info(quad_index_buffer) {
                indices = 0..(buffer_info.size / 4) as u32;
            } else {
                panic!("Expected buffer type.");
            }
        }

        // set global bindings
        self
            .draw_context
            .set_bind_groups_from_bindings(
                &mut self.draw, 
                &mut [])?;
        Ok(indices)
    }

    pub fn draw_sprite_atlas(&mut self, transform: &Transform, atlas: &Handle<TextureAtlas>, index: u32, color: Color) -> Result<(), DrawError>{
        let indices = self.set_mesh_attributes()?;
        self.draw_context.set_asset_bind_groups(&mut self.draw, atlas)?;
        
        let sprite = TextureAtlasSprite{
            index,
            color
        };

        let transform = transform.compute_matrix();
        let transform_buffer = self.draw_context.get_uniform_buffer(&transform).unwrap();
        let sprite_buffer = self.draw_context.get_uniform_buffer(&sprite).unwrap();
        let sprite_bind_group = BindGroup::build()
            .add_binding(0, transform_buffer)
            .add_binding(1, sprite_buffer)
            .finish();
        self.draw_context.create_bind_group_resource(2, &sprite_bind_group)?;
        self.draw.set_bind_group(2, &sprite_bind_group);
        self.draw.draw_indexed(indices.clone(), 0, 0..1);
        Ok(())
    }
    
    pub fn draw_sprite_atlas_instanced(&mut self, transform: &[Transform], atlas: &Handle<TextureAtlas>, indicies: &[u32]){
        self.draw_context.set_asset_bind_groups(&mut self.draw, atlas);
        todo!()
    }
}