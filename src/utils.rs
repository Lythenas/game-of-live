use amethyst::config::Config;
use log::error;
use std::path::Path;

/// Loads a config (or default if it fails to load).
///
/// Will also log an error if the file fails to load.
///
/// # Example
///
/// Usage e.g. in `SystemBundle.run` implementation:
///
/// ```ignore
/// world.insert(utils::load_config::<RunConfig>(
///     &self.config_path.join("run.ron"),
/// ));
/// ```
pub fn load_config<T>(path: &Path) -> T
where
    T: Config + Default,
{
    // The param can't be `impl AsRef<Path>` because then we can't specify the type easily at call
    // site. A second type param would also be ugly.

    let filename = path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("config");

    T::load(path).unwrap_or_else(|e| {
        error!("Failed to load {}: {}", filename, e);
        T::default()
    })
}
