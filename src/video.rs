use std::{cell::RefCell, rc::Rc};

use sdl2::sys::{SDL_CreateTexture, SDL_DestroyTexture, SDL_Rect, SDL_RenderCopy, SDL_Texture, SDL_UpdateTexture};

use crate::{Attribute, Component, Renderer, assets::{Asset, AssetExt}, util::Time};

use ffmpeg_next as ffmpeg;

use ffmpeg::{
    codec,
    format,
    frame,
    media,
    software::scaling::{context::Context as Scaler, flag::Flags},
    util::format::pixel::Pixel,
};

/// ## Description
/// Represents a single decoded RGBA video frame stored as an [SDL texture](SDL_Texture).
/// 
/// Used internally by the [Video2D] component to:
/// - Allocate a streaming texture
/// - Upload decoded FFmpeg frame data
/// - Render video frames efficiently
/// 
/// Each instance owns:
/// - An SDL streaming texture
/// - A raw pixel buffer
/// 
/// - **Item-Type**: Rendering Resource
/// 
/// ## Example
/// ```
/// let mut frame = VideoFrame2D::new(renderer, 1280, 720);
/// frame.data = decoded_pixels;
/// unsafe { frame.upload(); }
/// ```
#[derive(Debug, Clone)]
pub struct VideoFrame2D {
    pub texture: *mut SDL_Texture,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl VideoFrame2D {
    pub fn new(renderer: &mut Renderer, width: i32, height: i32) -> Self {
        unsafe {
            let texture = SDL_CreateTexture(
                renderer.get(),
                sdl2::sys::SDL_PixelFormatEnum::SDL_PIXELFORMAT_RGBA32 as u32,
                sdl2::sys::SDL_TextureAccess::SDL_TEXTUREACCESS_STREAMING as i32,
                width, 
                height
            );

            if texture.is_null() {
                panic!("[Pine] Failed to create video texture.");
            }

            Self {
                texture,
                width: width as u32,
                height: height as u32,
                data: vec![0; (width * height * 4) as usize],
            }
        }
    }

    pub unsafe fn upload(&mut self) {
        SDL_UpdateTexture(
            self.texture,
            std::ptr::null(),
            self.data.as_ptr() as *const _,
            (self.width * 4) as i32,
        );
    }
}

impl Drop for VideoFrame2D {
    fn drop(&mut self) {
        unsafe {
            if !self.texture.is_null() {
                SDL_DestroyTexture(self.texture);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub path: String,
}

impl MediaFile {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl AssetExt for MediaFile {}

impl Asset for MediaFile {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Asset> {
        Box::new((*self).clone())
    }
}

impl Attribute for MediaFile {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VideoSettings {
    HIDE_ON_FINISH,
    REPEATING,
    DEFAULT,
}

/// ## Description
/// Component that renders a video file onto the screen.
/// 
/// This component:
/// - Lazily initializes a [VideoDecoder]
/// - Decodes frames using [FFmpeg](ffmpeg)
/// - Uploads frames into [SDL textures](SDL_Texture)
/// - Renders them via the engine [renderer](Renderer)
/// 
/// Requires a [MediaFile] attribute to define the video path.
/// 
/// - **Item-Type**: Component
/// 
/// ## Example
/// ```
/// fn start() -> Result<(), RuntimeException> {
///     let video = make!(Video2D::new("IntroVideo", 30.0));
/// 
///     Engine::capture_any::<Video2D>(video.clone, |video| {
///         video.add_attribute(MediaFile::new("intro"), "IntroVideo_Media");
///     });
/// 
///     Engine::spawn(video)?;
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct Video2D {
    pub id: String,
    pub fps: f32,
    pub accumulator: f32,
    pub current_frame: Option<VideoFrame2D>,
    pub decoder: Option<VideoDecoder>,
    pub attributes: Vec<(String, Rc<RefCell<dyn Attribute>>)>,
    pub video_settings: VideoSettings,
}

impl Clone for VideoDecoder {
    fn clone(&self) -> Self {
        *self.clone_box()
    }
}

impl Video2D {
    pub fn new(id: impl Into<String>, fps: f32, options: VideoSettings) -> Self {
        Self {
            id: id.into(),
            fps,
            accumulator: 0.0,
            current_frame: None,
            decoder: None,
            attributes: Vec::new(),
            video_settings: options,
        }
    }

    pub fn new_from_media(id: impl Into<String> + std::fmt::Display + Clone, fps: f32, media_file: MediaFile, options: VideoSettings) -> Self {
        Self {
            id: id.clone().into(),
            fps,
            accumulator: 0.0,
            current_frame: None,
            decoder: None,
            attributes: vec![
                (format!("{id}_MediaFile"), Rc::new(RefCell::new(media_file)))
            ],
            video_settings: options
        }
    }

    pub(in crate) fn get_media_path(&self) -> Option<String> {
        for (_, attr) in &self.attributes {
            if let Some(media) = attr.borrow().as_any().downcast_ref::<MediaFile>() {
                return Some(media.path.clone())
            }
        }

        None
    }

    pub fn add_attribute(&mut self, attr: impl Attribute + 'static + Clone, id: impl Into<String>) {
        self.attributes.push((id.into(), Rc::new(RefCell::new(attr))));
    }

    pub fn restart(&mut self) {
        // Reset timing, playback state and frame
        self.accumulator = 0.0;
        self.current_frame = None;

        // recreate decoder
        if let Some(path) = self.get_media_path() {
            self.decoder = Some(VideoDecoder::new(&path));
        }
    }

    /// ## Safety
    /// unstable.
    #[deprecated]
    pub unsafe fn is_running(&self) -> bool {
        if let Some(decoder) = self.decoder.clone() {
            !decoder.ended
        } else {
            false
        }
    }
}

impl Component for Video2D {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
    fn component_id(&self) -> String {
        self.id.clone() 
    }
    fn component_type(&self) -> String {
        "Video2D".to_string()
    }
    fn get_attributes(&self) -> Vec<(String, Rc<RefCell<dyn Attribute>>)> {
        self.attributes.clone()
    }

    fn init(&self, handle: &mut crate::Handle) {}

    fn render(&mut self, renderer: &mut Renderer) {
        // Lazy decoder init
        if self.decoder.is_none() {
            if let Some(path) = self.get_media_path() {
                self.decoder = Some(VideoDecoder::new(&path));
            } else {
                return;
            }
        }

        let decoder = self.decoder.as_mut().unwrap();

        let frame_time = 1.0 / self.fps;
        self.accumulator += Time::delta();

        let mut new_frame = false;

        while self.accumulator >= frame_time {
            self.accumulator -= frame_time;

            if let Some(frame_data) = decoder.next_frame() {
                new_frame = true;

                if self.current_frame.is_none() {
                    self.current_frame = Some(VideoFrame2D::new(
                        renderer,
                        decoder.width as i32,
                        decoder.height as i32,
                    ));
                }

                if let Some(frame) = &mut self.current_frame {
                    frame.data = frame_data;
                    unsafe { frame.upload(); }
                }
            }
        }

        if decoder.ended && self.video_settings == VideoSettings::HIDE_ON_FINISH {
            return;
        } else if decoder.ended && self.video_settings == VideoSettings::REPEATING {
            self.restart();
        }

        // Always render last valid frame
        if let Some(frame) = &self.current_frame {
            unsafe {
                let dst = SDL_Rect {
                    x: 0,
                    y: 0,
                    w: frame.width as i32,
                    h: frame.height as i32,
                };

                SDL_RenderCopy(
                    renderer.get(),
                    frame.texture,
                    std::ptr::null(),
                    &dst,
                );
            }
        }
    }

}

/// ## Description
/// FFmpeg-backed video decoding utility used by [Video2D] component.
/// 
/// Responsible for:
/// - Opening a [media file](MediaFile)
/// - Selecting the best video stream
/// - Decoding frames
/// - Converting frames into RGBA format
/// 
/// ## Disclaimer
/// This type is internal to the video system and not meant for
/// direct use by game logic.
/// 
/// - **Item-Type**: Media Decoding Backend
/// 
/// ## Technical Info
/// Uses:
/// - `ffmpeg_next`
/// - Software scaling via `Scaler`
/// - RGBA pixel conversion
pub struct VideoDecoder {
    ictx: format::context::Input,
    decoder: codec::decoder::Video,
    stream_index: usize,
    scaler: Scaler,
    pub width: u32,
    pub height: u32,
    pub ended: bool,
}

impl VideoDecoder {
    pub fn new(path: &str) -> Self {
        ffmpeg::init().unwrap();

        let ictx = format::input(&path).unwrap();

        let input_stream =  ictx
            .streams()
            .best(media::Type::Video)
            .expect("No video stream found");

        let stream_index = input_stream.index();

        let context_decoder = codec::context::Context::from_parameters(input_stream.parameters())
            .unwrap();

        let decoder = context_decoder.decoder().video().unwrap();

        let width = decoder.width();
        let height = decoder.height();

        let scaler = Scaler::get(
            decoder.format(),
            width,
            height,
            Pixel::RGBA,
            width,
            height,
            Flags::BILINEAR
        ).unwrap();

        Self {
            ictx,
            decoder,
            stream_index,
            scaler,
            width,
            height,
            ended: false,
        }
    } 

    pub fn clone_box(&self) -> Box<Self> {
        Box::new((*self).clone())
    }

    pub fn next_frame(&mut self) -> Option<Vec<u8>> {
        for (stream, packet) in self.ictx.packets() {
            if stream.index() == self.stream_index {
                self.decoder.send_packet(&packet).ok()?;
            
                let mut decoded = frame::Video::empty();
            
                if self.decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = frame::Video::empty();
                    self.scaler.run(&decoded, &mut rgb_frame).ok()?;
                
                    let data = rgb_frame.data(0);
                    let linesize = rgb_frame.stride(0);
                
                    let mut buffer = vec![0u8; (self.width * self.height * 4) as usize];
                
                    for y in 0..self.height as usize {
                        let src_offset = y * linesize;
                        let dst_offset = y * self.width as usize * 4;
                    
                        buffer[dst_offset..dst_offset + self.width as usize * 4]
                            .copy_from_slice(
                                &data[src_offset..src_offset + self.width as usize * 4]
                            );
                    }
                
                    return Some(buffer);
                }
            }
        }
    
        // No more packets → flush decoder
        let mut decoded = frame::Video::empty();
    
        if self.decoder.receive_frame(&mut decoded).is_ok() {
            return None; // still frames pending
        }
    
        // Truly finished
        self.ended = true;
        None
    }
}