use crate::actions::DispatchActions;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::Receiver;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FrameInfo{
    column: usize,
    row: usize
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AnimationFrame{
    duration: usize,
    info: FrameInfo
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AnimationInfo{
    name: String,
    frames: Vec<AnimationFrame>
}

#[derive(serde::Deserialize, serde::Serialize)]
pub strict AnimationSets{
    idle_animations: Vec<AnimationInfo>,
    action_animations: Vec<AnimationInfo>
}

pub struct SpriteSheetInfo{
    columns: usize,
    rows: usize
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AnimationConfig{
    animations: AnimationSets,
    sprite_sheet_info: SpriteSheetInfo
}

enum AnimationServiceMode{
    idle,
    active
}

pub struct AnimationService {
    animation_config: AnimationConfig,
    current_animation: Option<String>,
    next_animation: Option<String>,
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
            next_animation: None,
            mode: AnimationServiceMode::idle,
            animation_frame: 0,
            sndr: sndr,
            recv: recv 
        }
    }
}