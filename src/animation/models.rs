#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FrameInfo{
    pub column: usize,
    pub row: usize
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct AnimationFrame{
    pub duration: usize,
    pub info: FrameInfo
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AnimationInfo{
    pub name: String,
    pub frames: Vec<AnimationFrame>
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AnimationSets{
    pub idle: Vec<AnimationInfo>,
    pub action: Vec<AnimationInfo>
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SpriteSheetInfo{
    pub columns: usize,
    pub rows: usize
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AnimationConfig{
    pub animations: AnimationSets,
    pub sprite_sheet_info: SpriteSheetInfo
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default, PartialEq)]
pub enum AnimationServiceMode{
    #[default]
    Idle,
    Active
}

