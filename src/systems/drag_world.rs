use amethyst::core::SystemBundle;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::input::InputHandler;
use amethyst::input::StringBindings;
use amethyst::winit::MouseButton;
use amethyst::window::ScreenDimensions;

use log::info;

use crate::states::loading::{INTERNAL_WIDTH, INTERNAL_HEIGHT};

#[derive(SystemDesc, Default, Debug)]
pub struct DragWorldSystem {
    // last_pos: Option<(f32, f32)>,
}

impl<'a> System<'a> for DragWorldSystem {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, ScreenDimensions>,
        ReadStorage<'a, ScreenParent>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (input, screen, parent_storage, mut transform_storage): Self::SystemData) {
        if input.action_is_down("move_world").unwrap_or(false) {
            let dx = input.axis_value("move_world_x").unwrap_or(0.0);
            let dy = input.axis_value("move_world_y").unwrap_or(0.0);

            let x_ratio = INTERNAL_WIDTH / screen.width();
            let y_ratio = INTERNAL_HEIGHT / screen.height();

            for (_, transform) in (&parent_storage, &mut transform_storage).join() {
                transform.move_left(dx * x_ratio);
                transform.move_up(dy * y_ratio);
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct DragWorldBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for DragWorldBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(DragWorldSystem::default(), "drag_world_system", &[]);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ScreenParent;

impl Component for ScreenParent {
    type Storage = NullStorage<Self>;
}
