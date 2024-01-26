use super::models::AnimationConfig;
use super::models::AnimationServiceMode;
use crate::actions::DispatchActions;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::Receiver;

struct AnimationService {
    animation_config: AnimationConfig,
    current_animation: Option<String>,
    mode: AnimationServiceMode,
    animation_frame: usize,
    sndr: Sender<DispatchActions>,
    recv: Receiver<DispatchActions>
}

impl AnimationService {
    /// Called once before the first frame.
    pub fn new(config : AnimationConfig, recv : Receiver<DispatchActions>, sndr : Sender<DispatchActions> ) -> Self {
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