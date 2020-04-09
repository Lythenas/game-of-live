use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
// use amethyst::ecs::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::input::StringBindings;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderFlat2D;
use amethyst::renderer::RenderToWindow;
use amethyst::renderer::RenderingBundle;
use amethyst::ui::RenderUi;
use amethyst::ui::UiBundle;
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst::utils::ortho_camera::CameraOrthoSystem;

mod states;
mod systems;

use states::game::BoardConfig;
use states::LoadingState;
use systems::CellBundle;
use systems::FpsDisplayBundle;
use systems::ControlBundle;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let configs_dir = app_root.join("config");

    let display_config = configs_dir.join("display.ron");
    let bindings_config = configs_dir.join("bindings.ron");
    let board_config = configs_dir.join("board.ron");

    let initial_state = LoadingState::default();

    let board_config = BoardConfig::load(&board_config)?;

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config)?;

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config)?.with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(rendering_bundle)?
        .with(CameraOrthoSystem::default(), "camera_ortho_system", &[])
        .with_bundle(ControlBundle)?
        .with_bundle(FpsCounterBundle)?
        .with_bundle(FpsDisplayBundle)?
        .with_bundle(CellBundle)?;

    let mut game = Application::build(assets_dir, initial_state)?
        .with_resource(board_config)
        .build(game_data)?;

    game.run();
    Ok(())
}
