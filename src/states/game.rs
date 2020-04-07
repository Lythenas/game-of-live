use std::collections::HashMap;
use std::collections::HashSet;

use amethyst::assets::Loader;
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::ui::Anchor;
use amethyst::ui::TtfFormat;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;

use serde::{Serialize, Deserialize};

use crate::systems::{Cell, CellState, Neighbors};

#[derive(Debug)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

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
                let text_transform = UiTransform::new(
                    format!("cell_{}_{}", x, y).to_string(),
                    Anchor::Middle,
                    Anchor::Middle,
                    x as f32 * board.tile_size,
                    y as f32 * board.tile_size,
                    1.,
                    board.tile_size,
                    board.tile_size,
                );

                let entity = world
                    .create_entity()
                    .with(Cell {
                        x,
                        y,
                        state: if alives.contains(&(x, y)) {
                            CellState::Alive
                        } else {
                            CellState::Dead
                        },
                    })
                    .with(text_transform)
                    .with(UiText::new(
                        font.clone(),
                        "".to_string(),
                        [1., 1., 1., 1.],
                        board.tile_size,
                    ))
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
