use crate::models::{environment, settings::Settings};

pub fn config() -> Result<Settings, anyhow::Error> {
    use config::{File, FileFormat};

    let env = environment::Environment::current_env();

    tracing::info!(%env, env_file=env.config_file().as_str(), "Loading config for environment");

    Ok(config::Config::builder()
        .add_source(File::new("config/base.yaml", FileFormat::Yaml))
        .add_source(File::new(env.config_file().as_str(), FileFormat::Yaml))
        .build()?
        .try_deserialize::<Settings>()?)
}
