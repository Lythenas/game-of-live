use std::collections::HashMap;
use std::collections::HashSet;

use amethyst::assets::Handle;
use amethyst::assets::Loader;
use amethyst::core::transform::Transform;
use amethyst::core::Hidden;
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;
use amethyst::ui::Anchor;
use amethyst::ui::TtfFormat;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;
use amethyst::core::transform::Parent;
use nalgebra::base::Vector3;

use serde::{Deserialize, Serialize};

use crate::systems::{Cell, CellState, Neighbors};

#[derive(Debug)]
pub struct GameState {
    pub sprite_sheet_handle: Handle<SpriteSheet>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let parent_entity = world.create_entity()
            .with(ScreenParent)
            .with(Transform::default())
            .build();

        // let font = world.read_resource::<Loader>().load(
        //     "font/Pixeled.ttf",
        //     TtfFormat,
        //     (),
        //     &world.read_resource(),
        // );

        let board: BoardConfig = (*world.read_resource::<BoardConfig>()).clone();

        let mut alives = HashSet::new();

        for y in 0..board.board.len() {
            let row = &board.board[y];
            for x in 0..row.len() {
                if row[x] == 1 {
                    alives.insert((x as i32, y as i32));
                }
            }
        }

        let mut entities = HashMap::new();

        for y in board.min_y..=board.max_y {
            for x in board.min_x..=board.max_x {
                let mut text_transform = Transform::default();
                text_transform.set_translation_xyz(
                    x as f32 * board.tile_size,
                    y as f32 * board.tile_size,
                    0.0,
                );
                text_transform.set_scale(Vector3::new(
                    board.tile_size / 8.0,
                    board.tile_size / 8.0,
                    1.0,
                ));

                // UiTransform::new(
                //     format!("cell_{}_{}", x, y).to_string(),
                //     Anchor::Middle,
                //     Anchor::Middle,
                //     x as f32 * board.tile_size,
                //     y as f32 * board.tile_size,
                //     1.,
                //     board.tile_size,
                //     board.tile_size,
                // );

                let alive = alives.contains(&(x, y));
                let entity = world
                    .create_entity()
                    .with(Cell {
                        x,
                        y,
                        state: if alive {
                            CellState::Alive
                        } else {
                            CellState::Dead
                        },
                    })
                    .with(Parent::new(parent_entity))
                    .with(text_transform)
                    .with(SpriteRender {
                        sprite_sheet: self.sprite_sheet_handle.clone(),
                        sprite_number: if alive { 0 } else { 1 },
                    })
                    .with(Tint(Srgba::new(0.5, 0.5, 0.5, 1.0)))
                    .build();

                entities.insert((x, y), entity);
            }
        }

        world.exec(|mut neighbors_store: WriteStorage<Neighbors>| {
            for y in board.min_y..=board.max_y {
                for x in board.min_x..=board.max_x {
                    let entity = entities.get(&(x, y)).unwrap();
                    let neighbors = Neighbors {
                        n: if y == board.min_y {
                            None
                        } else {
                            Some(entities.get(&(x, y - 1)).unwrap().clone())
                        },
                        ne: if y == board.min_y || x == board.max_x {
                            None
                        } else {
                            Some(entities.get(&(x + 1, y - 1)).unwrap().clone())
                        },
                        e: if x == board.max_x {
                            None
                        } else {
                            Some(entities.get(&(x + 1, y)).unwrap().clone())
                        },
                        se: if x == board.max_x || y == board.max_y {
                            None
                        } else {
                            Some(entities.get(&(x + 1, y + 1)).unwrap().clone())
                        },
                        s: if y == board.max_y {
                            None
                        } else {
                            Some(entities.get(&(x, y + 1)).unwrap().clone())
                        },
                        sw: if y == board.max_x || x == board.min_x {
                            None
                        } else {
                            Some(entities.get(&(x - 1, y + 1)).unwrap().clone())
                        },
                        w: if x == board.min_x {
                            None
                        } else {
                            Some(entities.get(&(x - 1, y)).unwrap().clone())
                        },
                        nw: if x == board.min_x || y == board.min_y {
                            None
                        } else {
                            Some(entities.get(&(x - 1, y - 1)).unwrap().clone())
                        },
                    };
                    neighbors_store.insert(*entity, neighbors).unwrap();
                }
            }
        });
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BoardConfig {
    tile_size: f32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    board: Vec<Vec<u8>>,
}

impl Default for BoardConfig {
    fn default() -> Self {
        Self {
            tile_size: 16.0,
            min_x: -20,
            max_x: 20,
            min_y: -20,
            max_y: 20,
            board: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ScreenParent;

impl Component for ScreenParent {
    type Storage = NullStorage<Self>;
}
