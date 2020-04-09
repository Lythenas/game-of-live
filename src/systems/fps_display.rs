use amethyst::assets::Loader;
use amethyst::core::timing::Time;
use amethyst::core::Hidden;
use amethyst::core::SystemBundle;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::input::InputEvent;
use amethyst::input::StringBindings;
use amethyst::shrev::EventChannel;
use amethyst::shrev::ReaderId;
use amethyst::ui::Anchor;
use amethyst::ui::TtfFormat;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;
use amethyst::utils::fps_counter::FpsCounter;

use super::UiConfig;

pub struct FpsText(pub Entity);

/// Display fps counter (sample_fps) in the top left corner.
///
/// Updates every 0.5 seconds to keep it readable and stops updating if the
/// text (FpsText component) is hidden.
#[derive(Debug, SystemDesc)]
pub struct FpsDisplaySystem {
    timer: f32,
    event_reader: ReaderId<InputEvent<StringBindings>>,
    visible: bool,
}

impl<'a> System<'a> for FpsDisplaySystem {
    type SystemData = (
        WriteStorage<'a, Hidden>,
        Read<'a, FpsCounter>,
        WriteStorage<'a, UiText>,
        ReadExpect<'a, FpsText>,
        ReadExpect<'a, Time>,
        Read<'a, UiConfig>,
    );

    fn run(
        &mut self,
        (mut hidden_storage, fps_counter, mut ui_text, fps_text, time, ui_config): Self::SystemData,
    ) {
        if self.visible != ui_config.show_fps {
            self.visible = ui_config.show_fps;
            if self.visible {
                hidden_storage.remove(fps_text.0);
            } else {
                hidden_storage.insert(fps_text.0, Hidden).unwrap();
            }
        }
        if self.visible {
            if let Some(text) = ui_text.get_mut(fps_text.0) {
                let time = time.delta_seconds();
                self.timer += time;
                if self.timer >= 0.5 {
                    self.timer = 0.0;
                    text.text = format!("{:.2}", fps_counter.sampled_fps()).to_string();
                }
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct FpsDisplayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for FpsDisplayBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        init_font(world);
        let event_reader = world.exec(
            |mut input_channel: Write<EventChannel<InputEvent<StringBindings>>>| {
                input_channel.register_reader()
            },
        );
        builder.add(
            FpsDisplaySystem {
                timer: 0.0,
                event_reader,
                visible: true,
            },
            "fps_display_system",
            &["fps_counter_system"],
        );
        Ok(())
    }
}

fn init_font(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let text_transform = UiTransform::new(
        "FPS_TEXT".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        0.,
        1.,
        200.,
        25.,
    );

    let mut ui_text = UiText::new(font.clone(), "".to_string(), [0., 1., 0., 0.5], 25.);
    ui_text.align = Anchor::MiddleLeft;

    let fps_text = world
        .create_entity()
        .with(text_transform)
        .with(ui_text)
        .build();
    world.insert(FpsText(fps_text));
}
