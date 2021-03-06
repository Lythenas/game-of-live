use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use amethyst::core::SystemBundle;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::input::InputEvent;
use amethyst::input::StringBindings;
use amethyst::prelude::*;
use amethyst::shrev::EventChannel;
use amethyst::shrev::ReaderId;
use nalgebra::base::Vector3;

use log::{debug, info};

use super::ScreenParent;
use crate::utils;

#[derive(SystemDesc)]
pub struct ControlSystem {
    event_reader: ReaderId<InputEvent<StringBindings>>,
}

impl<'a> System<'a> for ControlSystem {
    type SystemData = (
        Write<'a, RunConfig>,
        Write<'a, UiConfig>,
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
        ReadStorage<'a, ScreenParent>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (mut run_config, mut ui_config, event_channel, camera_storage, mut transform_storage): Self::SystemData,
    ) {
        for event in event_channel.read(&mut self.event_reader) {
            if let InputEvent::ActionPressed(action) = event {
                if action == "increase_speed" {
                    run_config.speed -= 0.1;
                    debug!("Increase speed");
                } else if action == "decrease_speed" {
                    run_config.speed += 0.1;
                    debug!("Decrease speed");
                } else if action == "toggle_pause" {
                    run_config.paused = !run_config.paused;
                    debug!("Toggle Pause ({})", run_config.paused);
                } else if action == "toggle_fps" {
                    ui_config.show_fps = !ui_config.show_fps;
                    debug!("Toggle fps ({})", ui_config.show_fps);
                } else if action == "scroll_left" {
                    // ui_config.camera_y -= 10;
                    for (_, transform) in (&camera_storage, &mut transform_storage).join() {
                        transform.move_left(50.0);
                    }
                } else if action == "scroll_right" {
                    // ui_config.camera_y += 10;
                    for (_, transform) in (&camera_storage, &mut transform_storage).join() {
                        transform.move_right(50.0);
                    }
                } else if action == "scroll_up" {
                    // ui_config.camera_x -= 10;
                    for (_, transform) in (&camera_storage, &mut transform_storage).join() {
                        transform.move_up(50.0);
                    }
                } else if action == "scroll_down" {
                    // ui_config.camera_x += 10;
                    for (_, transform) in (&camera_storage, &mut transform_storage).join() {
                        transform.move_down(50.0);
                    }
                } else if action == "zoom_in" {
                    for (_, transform) in (&camera_storage, &mut transform_storage).join() {
                        debug!("scale pre {:?}", transform.scale());
                        let scale = transform.scale()[0];
                        if scale < 10.0 {
                            let scale = scale * 1.2;
                            transform.set_scale(Vector3::new(scale, scale, 1.0));
                        }
                        debug!("scale post {:?}", transform.scale());
                    }
                } else if action == "zoom_out" {
                    for (_, transform) in (&camera_storage, &mut transform_storage).join() {
                        debug!("scale pre {:?}", transform.scale());
                        let scale = transform.scale()[0];
                        if scale > 0.1 {
                            let scale = scale / 1.2;
                            transform.set_scale(Vector3::new(scale, scale, 1.0));
                        }
                        debug!("scale post {:?}", transform.scale());
                    }
                }
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct ControlBundle {
    config_path: PathBuf,
}

impl ControlBundle {
    pub fn new(config_path: impl Into<PathBuf>) -> Self {
        Self {
            config_path: config_path.into(),
        }
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for ControlBundle {
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

        let system = ControlSystem { event_reader };

        world.insert(utils::load_config::<RunConfig>(
            &self.config_path.join("run.ron"),
        ));
        world.insert(utils::load_config::<UiConfig>(
            &self.config_path.join("ui.ron"),
        ));

        builder.add(system, "control_system", &[]);
        Ok(())
    }
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct RunConfig {
    pub paused: bool,
    /// Delay between cell simulation update (in seconds).
    pub speed: f32,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct UiConfig {
    pub show_fps: bool,
    pub camera_x: i32,
    pub camera_y: i32,
}
