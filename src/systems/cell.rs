use std::sync::mpsc::channel;

use amethyst::core::SystemBundle;
use amethyst::core::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::input::InputEvent;
use amethyst::input::StringBindings;
use amethyst::prelude::*;
use amethyst::shrev::EventChannel;
use amethyst::shrev::ReaderId;
use amethyst::ui::UiText;
use amethyst::core::Hidden;
use amethyst::renderer::SpriteRender;

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
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, UiText>,
        ReadExpect<'a, Time>,
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
        WriteExpect<'a, Paused>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut cell_storage,
            neighbors_storage,
            mut sprite_render_storage,
            mut ui_text,
            time,
            event_channel,
            mut paused,
        ): Self::SystemData,
    ) {
        for event in event_channel.read(&mut self.event_reader) {
            if let InputEvent::ActionPressed(action) = event {
                if action == "increase_speed" {
                    self.delay += 0.2;
                    info!("Delay {}", self.delay);
                } else if action == "decrease_speed" {
                    self.delay -= 0.2;
                    info!("Delay {}", self.delay);
                } else if action == "toggle_pause" {
                    paused.0 = !paused.0;
                    info!("Paused {:?}", paused.0);
                }
            }
        }

        if paused.0 {
            return;
        }

        self.timer += time.delta_seconds();

        if self.timer > self.delay {
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

#[derive(Debug, PartialEq, Eq)]
pub struct Paused(bool);

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

        world.insert(Paused(false));

        builder.add(system, "cell_system", &[]);
        Ok(())
    }
}
