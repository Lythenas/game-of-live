use amethyst::core::SystemBundle;
use amethyst::core::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::prelude::*;
use amethyst::ui::UiText;

use crate::systems::Cell;
use crate::systems::CellState;

#[derive(SystemDesc)]
pub struct CellDisplaySystem;

impl<'a> System<'a> for CellDisplaySystem {
    type SystemData = (ReadStorage<'a, Cell>, WriteStorage<'a, UiText>);

    fn run(&mut self, (cell_storage, mut text_storage): Self::SystemData) {
        for (cell, text) in (&cell_storage, &mut text_storage).join() {
            if cell.state == CellState::Alive {
                text.text = "#".to_string();
            } else {
                text.text = "-".to_string();
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct CellDisplayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CellDisplayBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // TODO ui as dep
        builder.add(CellDisplaySystem, "cell_display_system", &["cell_system"]);
        Ok(())
    }
}
