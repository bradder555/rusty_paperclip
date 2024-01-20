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
pub struct AnimationSets{
    idle_animations: Vec<AnimationInfo>,
    action_animations: Vec<AnimationInfo>
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SpriteSheetInfo{
    pub columns: usize,
    pub rows: usize
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AnimationConfig{
    animations: AnimationSets,
    sprite_sheet_info: SpriteSheetInfo
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum AnimationServiceMode{
    idle,
    active
}

