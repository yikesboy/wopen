use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Backend {
    Auto,
    Sway,
    Hyprland,
}
