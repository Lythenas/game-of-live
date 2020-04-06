use std::collections::HashMap;
use std::collections::HashSet;

use amethyst::assets::Loader;
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::ui::Anchor;
use amethyst::ui::TtfFormat;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;

use crate::systems::{Cell, CellState, Neighbors};

const SIZE: f32 = 50.0;
const BOARD_X_MIN: i32 = -5;
const BOARD_X_MAX: i32 = 5;
const BOARD_Y_MIN: i32 = -5;
const BOARD_Y_MAX: i32 = 5;

#[derive(Debug, Default)]
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

        let mut alives = HashSet::new();
        alives.insert((1, 0));
        alives.insert((2, 1));
        alives.insert((0, 2));
        alives.insert((1, 2));
        alives.insert((2, 2));

        let mut entities = HashMap::new();

        for y in BOARD_Y_MIN..=BOARD_Y_MAX {
            for x in BOARD_X_MIN..=BOARD_X_MAX {
                let text_transform = UiTransform::new(
                    format!("cell_{}_{}", x, y).to_string(),
                    Anchor::Middle,
                    Anchor::Middle,
                    x as f32 * SIZE,
                    y as f32 * SIZE,
                    1.,
                    SIZE,
                    SIZE,
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
                        SIZE,
                    ))
                    .build();

                entities.insert((x, y), entity);
            }
        }

        world.exec(|mut neighbors_store: WriteStorage<Neighbors>| {
            for y in BOARD_Y_MIN..=BOARD_Y_MAX {
                for x in BOARD_X_MIN..=BOARD_X_MAX {
                    let entity = entities.get(&(x, y)).unwrap();
                    let neighbors = Neighbors {
                        n: if y == BOARD_Y_MIN {
                            None
                        } else {
                            Some(entities.get(&(x, y - 1)).unwrap().clone())
                        },
                        ne: if y == BOARD_Y_MIN || x == BOARD_X_MAX {
                            None
                        } else {
                            Some(entities.get(&(x + 1, y - 1)).unwrap().clone())
                        },
                        e: if x == BOARD_X_MAX {
                            None
                        } else {
                            Some(entities.get(&(x + 1, y)).unwrap().clone())
                        },
                        se: if x == BOARD_X_MAX || y == BOARD_Y_MAX {
                            None
                        } else {
                            Some(entities.get(&(x + 1, y + 1)).unwrap().clone())
                        },
                        s: if y == BOARD_Y_MAX {
                            None
                        } else {
                            Some(entities.get(&(x, y + 1)).unwrap().clone())
                        },
                        sw: if y == BOARD_Y_MAX || x == BOARD_X_MIN {
                            None
                        } else {
                            Some(entities.get(&(x - 1, y + 1)).unwrap().clone())
                        },
                        w: if x == BOARD_X_MIN {
                            None
                        } else {
                            Some(entities.get(&(x - 1, y)).unwrap().clone())
                        },
                        nw: if x == BOARD_X_MIN || y == BOARD_Y_MIN {
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

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
