use std::io::Cursor;
use std::sync::Arc;
use std::sync::Mutex;

use std::time::Duration;

use rand::Rng;

use super::models::AnimationConfig;
use super::models::AnimationFrame;
use super::models::AnimationInfo;
use super::models::AnimationServiceMode;
use crate::actions::DispatchActions;

use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;
use egui::Ui;
use tokio::sync::broadcast::Sender;

use egui::scroll_area::ScrollAreaOutput;


#[derive(Clone)]
struct AnimationState {
    current_animation: Option<AnimationInfo>,
    current_frame_index: usize,
    current_frame_info: Option<AnimationFrame>,
    mode: AnimationServiceMode
    
}

#[derive(Clone)]
pub struct AnimationService {
    animation_config: AnimationConfig,
    sndr: Sender<DispatchActions>,
    state: Arc<Mutex<AnimationState>>,
    sprite_height: usize,
    sprite_width: usize,
    texture_handle: TextureHandle,
    should_run: bool
}

fn load_image_as_color_image(bytes: &Vec<u8> ) -> ColorImage {
    let mut c = Cursor::new(bytes);
    c.set_position(0);

    let im_buff = image::io::Reader::new(
            &mut c
        ).with_guessed_format()
        .expect("can't guess format")
        .decode()
        .expect("can't decode image");

    let im_buff = im_buff.to_rgba8();
    let pix = im_buff.as_flat_samples();
    ColorImage::from_rgba_unmultiplied(
        [im_buff.width() as _, 
        im_buff.height() as _], 
        pix.as_slice()
    )
}

impl AnimationService {
    /// Called once before the first frame.
    pub fn new(
        ctx: Context,
        config_data :  String,
        image_data: Vec<u8>,
        sndr : Sender<DispatchActions>
    ) -> Self {
        let config: AnimationConfig = serde_yaml::from_str(&config_data)
            .expect("trouble reading config file");

        let texture = ctx.load_texture(
            "clippit_sprite_sheet", 
            load_image_as_color_image(&image_data), 
            Default::default()
        );

        let im_size = texture.size_vec2();
        let width = (im_size.x as usize) / config.sprite_sheet_info.columns;
        let height = (im_size.y as usize) / config.sprite_sheet_info.rows;

        AnimationService {
            animation_config : config,
            sndr: sndr,
            sprite_width: width,
            sprite_height: height,
            texture_handle: texture,
            should_run: false,
            state: Arc::from(
                Mutex::from(
                    AnimationState{
                        current_animation: None,
                        mode: AnimationServiceMode::Idle,
                        current_frame_index: 0,
                        current_frame_info: None,
                    }
                )
            )
        }
    }

    pub fn start(&mut self){

        if self.should_run == true {return}
        self.should_run = true;
        
        let state = self.state.clone();
        let config = self.animation_config.clone();
        let sndr = self.sndr.clone();

        tokio::spawn(async move {
            let mut receiver = sndr.subscribe();

            loop{
                let duration;
                let mode = match receiver.try_recv() {
                    Ok(DispatchActions::AskQuestion(_question)) => {
                        Some(AnimationServiceMode::Active)
                    },
                    Ok(DispatchActions::RespondToQuestion(_answer)) => {
                        Some(AnimationServiceMode::Idle)
                    },
                    _ => None
                };

                {
                    // internal state
                    let mut i_s = state.lock().unwrap();

                    match &mode {
                        Some(x) => {
                            i_s.mode = x.clone();
                            // short circuit the animation
                            i_s.current_animation = None;
                            continue 
                        },
                        _ => ()
                    }
            
                    // if no animation selected, pick one at random (depending on the current mode)
                    if i_s.current_animation.is_none() {
                        let animation: Option<AnimationInfo>;
                        let rand = {
                            let mut rng = rand::thread_rng();
                            let r = rng.gen::<u32>() as usize;
                            drop(rng);
                            r
                        };
                        let ai;
                        let animation = 
                            match i_s.mode {
                                AnimationServiceMode::Idle => {
                                    ai = rand % config.animations.idle.len();
                                    config.animations.idle.get(ai).cloned()
                                },
                                _ => {
                                    ai = rand % config.animations.action.len();
                                    config.animations.action.get(ai).cloned()
                                }
                            };

                        let _ = sndr.send(
                            DispatchActions::NewAnimationStarted(
                                animation.clone().unwrap().name
                            )
                        ).unwrap_or_default(); // needs to be expect

                        i_s.current_animation = animation;
                        i_s.current_frame_index = 0;
                    }
            
                    let animation = i_s.current_animation.clone().unwrap();
            
                    // guard against reading non-existent frame
                    if i_s.current_frame_index > animation.frames.len() - 1 {
                        i_s.current_animation = None;
                        continue;
                    }
                    
                    let frame_info = &animation.frames[i_s.current_frame_index].clone();
                    i_s.current_frame_info = Some(frame_info.clone());
                    i_s.current_frame_index += 1;
                    drop(i_s); // free up mutex then wait for the duration
                    duration = frame_info.duration.clone();
                }
                let _ = sndr.send(DispatchActions::NewFrameToRender);
                tokio::time::sleep(Duration::from_millis(duration as u64)).await;
            }
        });
        
    }

    pub fn render_animation(&self, ui: &mut Ui) -> ScrollAreaOutput<()>{
        let frame;
        {
        let state_ = self.state.lock().unwrap();
        frame = state_.current_frame_info.clone().unwrap_or_default();
        }

        let vert_scroll_off = 1.0 + (frame.info.row * self.sprite_height) as f32;
        let hoz_scroll_off = 1.0 + (frame.info.column * self.sprite_width) as f32;

        egui::ScrollArea::both()
            .max_height((self.sprite_height - 5) as f32 )
            .max_width((self.sprite_width - 5) as f32 )
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .enable_scrolling(false)
            .auto_shrink(false)
            .vertical_scroll_offset(vert_scroll_off)
            .horizontal_scroll_offset(hoz_scroll_off)
            .show(ui, |ui|{
            ui.add(
                egui::Image::from_texture(&self.texture_handle)
            );
        })     
    }


}