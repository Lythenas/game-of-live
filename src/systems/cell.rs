use amethyst::core::SystemBundle;
use amethyst::core::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::prelude::*;

use log::info;

#[derive(SystemDesc)]
pub struct CellSystem {
    timer: f32,
}

impl<'a> System<'a> for CellSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Cell>,
        ReadStorage<'a, Neighbors>,
        ReadExpect<'a, Time>,
    );

    fn run(&mut self, (entities, mut cell_storage, neighbors_storage, time): Self::SystemData) {
        self.timer += time.delta_seconds();

        if self.timer > 1.0 {
            self.timer = 0.0;

            let mut kill_cells = Vec::new();
            let mut revive_cells = Vec::new();

            info!("------------");
            for (entity, cell, neighbors) in (&entities, &cell_storage, &neighbors_storage).join() {
                let alive_neighbors = neighbors.get_num_alive(&cell_storage);
                if cell.state == CellState::Dead {
                    // dead cell
                    if alive_neighbors == 3 {
                        revive_cells.push(entity);
                    }
                    info!("{},{} has {} heighbors. reviving", cell.x, cell.y, alive_neighbors);
                } else {
                    // alive cell
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        kill_cells.push(entity);
                    }
                    info!("{},{} has {} heighbors. killing", cell.x, cell.y, alive_neighbors);
                }
            }

            for entity in kill_cells {
                cell_storage
                    .get_mut(entity)
                    .map(|c| c.state = CellState::Dead);
            }
            for entity in revive_cells {
                cell_storage
                    .get_mut(entity)
                    .map(|c| c.state = CellState::Alive);
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
        return [self.n, self.ne, self.e, self.se, self.s, self.sw, self.w, self.nw]
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
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(CellSystem { timer: 0.0 }, "cell_system", &[]);
        Ok(())
    }
}
