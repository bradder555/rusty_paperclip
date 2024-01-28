use std::sync::Arc;
use std::sync::Mutex;

use std::thread;
use std::time::Duration;

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

use rand::seq::SliceRandom;
use rand::thread_rng;


#[derive(Clone)]
struct AnimationState {
    should_run: bool,
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
    texture_handle: TextureHandle
}

fn load_image_as_color_image(filepath:&str) -> ColorImage {
    let image = image::io::Reader::open(filepath)
    .expect("problem loading image")
    .decode()
    .expect("problem decoding image, panic");

    let im_buff = image.to_rgba8();
    let pix = im_buff.as_flat_samples();
    ColorImage::from_rgba_unmultiplied([im_buff.width() as _, im_buff.height() as _], pix.as_slice())
}

fn run(
    state: Arc<Mutex<AnimationState>>, 
    config: AnimationConfig,
    sndr: Sender<DispatchActions>
){
    loop{
        // internal state
        let mut i_s = state.lock().unwrap();

        // bail if stop is called
        if !i_s.should_run {break}

        // if no animation selected, pick one at random (depending on the current mode)
        if i_s.current_animation.is_none() {
            let mut rng = thread_rng();
            let animation: Option<AnimationInfo>;
            if i_s.mode == AnimationServiceMode::Idle{
                animation = config.animations.idle.choose(&mut rng).cloned();
            } else {
                animation = config.animations.action.choose(&mut rng).cloned();
            }
            i_s.current_animation = Some(animation.expect("no animation"));
            i_s.current_frame_index = 0;
        }
  
        let animation = i_s.current_animation.as_ref().unwrap();

        // guard against reading non-existent frame
        if i_s.current_frame_index > animation.frames.len() - 1 {
            i_s.current_animation = None;
            continue;
        }
        
        let frame_info = &animation.frames[i_s.current_frame_index].clone();
        i_s.current_frame_info = Some(frame_info.clone());
        i_s.current_frame_index += 1;
        drop(i_s); // free up mutex then wait for the duration
        let duration = frame_info.duration.clone();
        let _ = sndr.send(DispatchActions::UpdateFrame);
        thread::sleep(Duration::from_millis(duration as u64));
    }
}

impl AnimationService {
    /// Called once before the first frame.
    pub fn new(
        ctx: Context,
        config_file : &str,
        image_file: &str,
        sndr : Sender<DispatchActions> 
    ) -> Self {

        let file = std::fs::File::open(config_file).expect("trouble reading config file");
        let config : AnimationConfig = serde_yaml::from_reader(file).expect("trouble parsing config");
        let texture = ctx.load_texture(
            "clippit_sprite_sheet", 
            load_image_as_color_image(image_file), 
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
            state: Arc::from(
                Mutex::from(
                    AnimationState{
                        should_run: false,
                        current_animation: None,
                        mode: AnimationServiceMode::Idle,
                        current_frame_index: 0,
                        current_frame_info: None,
                    }
                )
            )
        }
    }

    pub fn start(&self){
        let mut state = self.state.lock().unwrap();

        if state.should_run == true {return}
        state.should_run = true;
        drop(state);

        
        let conf = self.animation_config.clone();
        let sndr = self.sndr.clone();
        let state = self.state.clone();

        thread::spawn(move || {
            run(
                state, 
                conf,
                sndr
            )
        });
        
    }

    pub fn stop(&self){
        let mut state = self.state.lock().unwrap();
        state.should_run = false;
    }

    pub fn set_mode(&self, mode: AnimationServiceMode){
        let mut state = self.state.lock().unwrap();
        state.mode = mode;
    }

    pub fn get_current_animation_name(&self) -> String{
        let state = self.state.lock().unwrap();
        state.current_animation.as_ref().unwrap().name.clone()
    }

    pub fn render_animation(&self, ui: &mut Ui){
        let state = self.state.lock().unwrap();
        let frame = state.current_frame_info.clone().unwrap_or_default();
        drop(state);

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
        });        
    }


}