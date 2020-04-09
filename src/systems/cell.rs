use std::sync::mpsc::channel;

use amethyst::core::SystemBundle;
use amethyst::core::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::renderer::SpriteRender;

use super::RunConfig;

#[derive(Debug, Default, SystemDesc)]
pub struct CellSystem {
    timer: f32,
}

impl<'a> System<'a> for CellSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Cell>,
        ReadStorage<'a, Neighbors>,
        WriteStorage<'a, SpriteRender>,
        ReadExpect<'a, Time>,
        Read<'a, RunConfig>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut cell_storage,
            neighbors_storage,
            mut sprite_render_storage,
            time,
            run_config,
        ): Self::SystemData,
    ) {
        if run_config.paused {
            return;
        }

        self.timer += time.delta_seconds();

        if self.timer > run_config.speed {
            self.timer = 0.0;

            // iterate over all cells in parallel and use channels to collect
            // which cells to kill or revive
            let (kill_sender, kill_receiver) = channel();
            let (revive_sender, revive_receiver) = channel();

            (&entities, &cell_storage, &neighbors_storage)
                .par_join()
                .for_each_with(
                    (kill_sender, revive_sender),
                    |(kill, revive), (entity, cell, neighbors)| {
                        let alive_neighbors = neighbors.get_num_alive(&cell_storage);
                        if cell.state == CellState::Dead {
                            // dead cell
                            if alive_neighbors == 3 {
                                revive.send(entity).unwrap();
                            }
                        } else {
                            // alive cell
                            if alive_neighbors < 2 || alive_neighbors > 3 {
                                kill.send(entity).unwrap();
                            }
                        }
                    },
                );

            let kill_cells: Vec<_> = kill_receiver.iter().collect();
            let revive_cells: Vec<_> = revive_receiver.iter().collect();

            for entity in kill_cells {
                cell_storage
                    .get_mut(entity)
                    .map(|c| c.state = CellState::Dead);
                // ui_text.get_mut(entity).map(|t| t.text = "-".to_string());
                // hidden_storage.insert(entity, Hidden);
                sprite_render_storage.get_mut(entity).map(|s| s.sprite_number = 1);
            }
            for entity in revive_cells {
                cell_storage
                    .get_mut(entity)
                    .map(|c| c.state = CellState::Alive);
                // ui_text.get_mut(entity).map(|t| t.text = "#".to_string());
                // hidden_storage.remove(entity);
                sprite_render_storage.get_mut(entity).map(|s| s.sprite_number = 0);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub state: CellState,
}

impl Component for Cell {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Neighbors {
    pub n: Option<Entity>,
    pub ne: Option<Entity>,
    pub e: Option<Entity>,
    pub se: Option<Entity>,
    pub s: Option<Entity>,
    pub sw: Option<Entity>,
    pub w: Option<Entity>,
    pub nw: Option<Entity>,
}

impl Component for Neighbors {
    type Storage = VecStorage<Self>;
}

impl Neighbors {
    fn get_num_alive<'a>(&self, cell_storage: &WriteStorage<'a, Cell>) -> usize {
        return [
            self.n, self.ne, self.e, self.se, self.s, self.sw, self.w, self.nw,
        ]
        .iter()
        .map(|n| n.as_ref().and_then(|e| cell_storage.get(*e)))
        .filter(|c| match *c {
            Some(Cell {
                state: CellState::Alive,
                ..
            }) => true,
            _ => false,
        })
        .count();
    }
}

#[derive(Default, Debug)]
pub struct CellBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CellBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(CellSystem::default(), "cell_system", &[]);
        Ok(())
    }
}

