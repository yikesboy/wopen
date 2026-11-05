use clap::ValueEnum;
use std::env;
use std::fmt::Error;
use std::process::Command;
use thiserror::Error;

#[derive(ValueEnum, Clone, Debug)]
pub enum Backend {
    Auto,
    Sway,
    Hyprland,
}

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("both sway and hyprland were detected")]
    AmbigiousEnvironment,
}

impl Backend {
    pub fn resolve(self) -> Result<Self, BackendError> {
        match self {
            Backend::Auto => Self::detect(),
            backend => Ok(backend),
        }
    }

    fn detect() -> Result<Self, BackendError> {
        let is_sway_active = env::var_os("SWAYSOCK").is_some();
        let is_hyprland_active = env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some();

        match (is_sway_active, is_hyprland_active) {
            (true, false) => Ok(Self::Sway),
            (false, true) => Ok(Self::Hyprland),
            (false, false) => Ok(Self::Auto),
            (true, true) => Err(BackendError::AmbigiousEnvironment),
        }
    }
}
