use amethyst::assets::AssetStorage;
use amethyst::assets::Handle;
use amethyst::assets::Loader;
use amethyst::assets::ProgressCounter;
use amethyst::core::transform::Transform;
use amethyst::core::Hidden;
use amethyst::core::Time;
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::renderer::Camera;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;
use amethyst::ui::Anchor;
use amethyst::ui::TtfFormat;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;
use amethyst::utils::ortho_camera::CameraNormalizeMode;
use amethyst::utils::ortho_camera::CameraOrtho;
use amethyst::utils::ortho_camera::CameraOrthoWorldCoordinates;

use log::info;

use super::GameState;

#[derive(Debug, Default)]
pub struct LoadingState {
    time: f32,
    progress: ProgressCounter,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

struct LoadingText(Entity);
struct TimerText(Entity);

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        init_font(world, self);

        {
            let loader = world.read_resource::<Loader>();

            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            let texture_handle = loader.load(
                "sprites/spritesheet.png",
                ImageFormat::default(),
                &mut self.progress,
                &texture_storage,
            );

            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            let sprite_sheet_handle = loader.load(
                "sprites/spritesheet.ron",
                SpriteSheetFormat(texture_handle),
                &mut self.progress,
                &sprite_sheet_store,
            );

            self.sprite_sheet_handle.replace(sprite_sheet_handle);
        }

        initialise_camera(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(loading_text) = data.world.remove::<LoadingText>() {
            data.world
                .delete_entity(loading_text.0)
                .expect("Could not remove loading text");
        }
        if let Some(timer_text) = data.world.remove::<TimerText>() {
            data.world
                .delete_entity(timer_text.0)
                .expect("Could not remove timer text");
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress.is_complete() {
            data.world.exec(
                |(mut ui_text, loading_text): (WriteStorage<UiText>, ReadExpect<LoadingText>)| {
                    if let Some(text) = ui_text.get_mut(loading_text.0) {
                        text.text = "Done".to_string();
                    }
                },
            );

            {
                let time = data.world.fetch::<Time>();
                self.time += time.delta_seconds();
            }

            data.world.exec(
                |(mut ui_text, timer_text, mut hidden_storage): (
                    WriteStorage<UiText>,
                    ReadExpect<TimerText>,
                    WriteStorage<Hidden>,
                )| {
                    hidden_storage.remove(timer_text.0);
                    if let Some(text) = ui_text.get_mut(timer_text.0) {
                        text.text = format!("{:.2}", self.time).to_string();
                    }
                },
            );

            if self.time >= 2.0 {
                info!("Switching to game state");
                Trans::Replace(Box::new(GameState {
                    sprite_sheet_handle: self
                        .sprite_sheet_handle
                        .take()
                        .expect("sprite_sheet_handle does not exist"),
                }))
            } else {
                Trans::None
            }
        } else {
            data.world.exec(
                |(mut ui_text, loading_text): (WriteStorage<UiText>, ReadExpect<LoadingText>)| {
                    if let Some(text) = ui_text.get_mut(loading_text.0) {
                        text.text = format!(
                            "Loading: {} / {}",
                            self.progress.num_finished(),
                            self.progress.num_assets()
                        )
                        .to_string();
                    }
                },
            );

            Trans::None
        }
    }
}

fn init_font(world: &mut World, state: &mut LoadingState) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        &mut state.progress,
        &world.read_resource(),
    );

    let text_transform = UiTransform::new(
        "LOADING_TEXT".to_string(),
        Anchor::BottomMiddle,
        Anchor::BottomMiddle,
        0.,
        0.,
        1.,
        500.,
        50.,
    );

    let loading_text = world
        .create_entity()
        .with(text_transform)
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();
    world.insert(LoadingText(loading_text));

    let timer_text_transform = UiTransform::new(
        "TIMER_TEXT".to_string(),
        Anchor::Middle,
        Anchor::TopMiddle,
        0.,
        0.,
        1.,
        500.,
        50.,
    );

    let timer_text = world
        .create_entity()
        .with(timer_text_transform)
        .with(UiText::new(
            font.clone(),
            "timer".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .with(Hidden)
        .build();
    world.insert(TimerText(timer_text));
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(DISPLAY_WIDTH, DISPLAY_HEIGHT))
        .with(transform)
        .with(CameraOrtho::new(
            CameraNormalizeMode::Contain,
            CameraOrthoWorldCoordinates {
                left: -INTERNAL_WIDTH / 2.0,
                right: INTERNAL_WIDTH / 2.0,
                bottom: -INTERNAL_HEIGHT / 2.0,
                top: INTERNAL_HEIGHT / 2.0,
            },
        ))
        .build();
}

pub const DISPLAY_WIDTH: f32 = 1920.0;
pub const DISPLAY_HEIGHT: f32 = 1080.0;

pub const INTERNAL_WIDTH: f32 = 3000.0;
pub const INTERNAL_HEIGHT: f32 = 3000.0;

