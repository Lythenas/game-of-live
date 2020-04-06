use amethyst::assets::Loader;
use amethyst::core::timing::Time;
use amethyst::core::Hidden;
use amethyst::core::SystemBundle;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::input::InputEvent;
use amethyst::input::StringBindings;
use amethyst::prelude::*;
use amethyst::ui::Anchor;
use amethyst::ui::TtfFormat;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;
use amethyst::utils::fps_counter::FpsCounter;
use amethyst::shrev::EventChannel;
use amethyst::shrev::ReaderId;

use log::info;

pub struct FpsText(pub Entity);

/// Display fps counter (sample_fps) in the top left corner.
///
/// Updates every 0.5 seconds to keep it readable and stops updating if the
/// text (FpsText component) is hidden.
#[derive(Debug, SystemDesc)]
pub struct FpsDisplaySystem {
    timer: f32,
    event_reader: ReaderId<InputEvent<StringBindings>>,
}

impl<'a> System<'a> for FpsDisplaySystem {
    type SystemData = (
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
        WriteStorage<'a, Hidden>,
        Read<'a, FpsCounter>,
        WriteStorage<'a, UiText>,
        ReadExpect<'a, FpsText>,
        ReadExpect<'a, Time>,
    );

    fn run(
        &mut self,
        (input_events, mut hidden_storage, fps_counter, mut ui_text, fps_text, time): Self::SystemData,
    ) {
        for event in input_events.read(&mut self.event_reader) {
            // info!("{:?}", event);
            if let InputEvent::ActionPressed(action) = event {
                if action == "toggle_fps" {
                    if hidden_storage.contains(fps_text.0) {
                        hidden_storage.remove(fps_text.0);
                    } else {
                        hidden_storage
                            .insert(fps_text.0, Hidden)
                            .expect("Hidden component not found");
                    }
                }
            }
        }
        if !hidden_storage.contains(fps_text.0) {
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
