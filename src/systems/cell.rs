use amethyst::core::SystemBundle;
use amethyst::core::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::prelude::*;
use amethyst::input::InputEvent;
use amethyst::input::StringBindings;
use amethyst::shrev::EventChannel;
use amethyst::shrev::ReaderId;

use log::info;

#[derive(SystemDesc)]
pub struct CellSystem {
    timer: f32,
    /// Delay between cell simulation update (in seconds).
    delay: f32,
    event_reader: ReaderId<InputEvent<StringBindings>>,
}

impl<'a> System<'a> for CellSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Cell>,
        ReadStorage<'a, Neighbors>,
        ReadExpect<'a, Time>,
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
    );

    fn run(&mut self, (entities, mut cell_storage, neighbors_storage, time, event_channel): Self::SystemData) {
        for event in event_channel.read(&mut self.event_reader) {
            if let InputEvent::ActionPressed(action) = event {
                if action == "increase_speed" {
                    self.delay += 0.2;
                    info!("Delay {}", self.delay);
                } else if action == "decrease_speed" {
                    self.delay -= 0.2;
                    info!("Delay {}", self.delay);
                }
            }
        }

        self.timer += time.delta_seconds();

        if self.timer > self.delay {
            self.timer = 0.0;

            let mut kill_cells = Vec::new();
            let mut revive_cells = Vec::new();

            for (entity, cell, neighbors) in (&entities, &cell_storage, &neighbors_storage).join() {
                let alive_neighbors = neighbors.get_num_alive(&cell_storage);
                if cell.state == CellState::Dead {
                    // dead cell
                    if alive_neighbors == 3 {
                        revive_cells.push(entity);
                    }
                } else {
                    // alive cell
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        kill_cells.push(entity);
                    }
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
        let event_reader = world.exec(
            |mut input_channel: Write<EventChannel<InputEvent<StringBindings>>>| {
                input_channel.register_reader()
            },
        );
        let system = CellSystem {
            timer: 0.0,
            delay: 1.0,
            event_reader,
        };

        builder.add(system, "cell_system", &[]);
        Ok(())
    }
}
