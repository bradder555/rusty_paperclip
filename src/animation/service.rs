use std::io::Error;

use super::models::AnimationConfig;
use super::models::AnimationServiceMode;
use crate::actions::DispatchActions;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::Receiver;

pub struct AnimationService {
    animation_config: AnimationConfig,
    current_animation: Option<String>,
    mode: AnimationServiceMode,
    animation_frame: usize,
    sndr: Sender<DispatchActions>,
    recv: Receiver<DispatchActions>
}

impl AnimationService {
    /// Called once before the first frame.
    pub fn new(config_file : &str, recv : Receiver<DispatchActions>, sndr : Sender<DispatchActions> ) -> Self {

        let file = std::fs::File::open(config_file).expect("trouble reading config file");
        let config = serde_yaml::from_reader(file).expect("trouble parsing config");

        AnimationService {
            animation_config : config,
            current_animation: None,
            mode: AnimationServiceMode::Idle,
            animation_frame: 0,
            sndr: sndr,
            recv: recv 
        }
    }
}