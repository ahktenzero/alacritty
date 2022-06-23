use std::time::Duration;

use crate::config::bindings::ModsWrapper;
use alacritty_config_derive::ConfigDeserialize;
use glutin::event::ModifiersState;

#[derive(ConfigDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Mouse {
    pub double_click: ClickHandler,
    pub triple_click: ClickHandler,
    pub hide_when_typing: bool,
    #[config(deprecated = "use `hints` section instead")]
    pub url: Option<serde_yaml::Value>,
    pub rectangle_select: ModsWrapper,
}

#[derive(ConfigDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ClickHandler {
    threshold: u16,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            double_click: Default::default(),
            triple_click: Default::default(),
            hide_when_typing: false,
            url: Default::default(),
            rectangle_select: ModsWrapper(ModifiersState::CTRL),
        }
    }
}

impl Default for ClickHandler {
    fn default() -> Self {
        Self { threshold: 300 }
    }
}

impl ClickHandler {
    pub fn threshold(&self) -> Duration {
        Duration::from_millis(self.threshold as u64)
    }
}
