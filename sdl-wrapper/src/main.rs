#![allow(non_camel_case_types, unused, non_snake_case)]

use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;

const SDL_INIT_VIDEO: u32 = 0x00000020;

const SDL_WINDOW_HIDDEN: u32 = 0x00000008;
const SDL_WINDOW_RESIZABLE: u32 = 0x00000020;

const SDL_RENDERER_ACCELERATED: u32 = 0x00000002;

const SDL_QUIT: u32 = 0x100;

const SCREEN_HEIGHT: i32 = 800;
const SCREEN_WIDTH: i32 = 600;

type Raw_SDL_Window = c_void;
type Raw_SDL_Renderer = c_void;

#[repr(C, packed)]
struct SDL_Event {
    r#type: u32,
    padding: [u8; 52],
}

#[link(name = "SDL2")]
extern "C" {
    fn SDL_Init(flags: u32) -> i32;
    fn SDL_Quit();
    fn SDL_GetError() -> *const c_char;
    fn SDL_CreateWindow(
        title: *const c_char,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: u32,
    ) -> *mut Raw_SDL_Window;
    fn SDL_ShowWindow(window: *mut Raw_SDL_Window);
    fn SDL_DestroyWindow(window: *mut Raw_SDL_Window);
    fn SDL_CreateRenderer(
        window: *mut Raw_SDL_Window,
        index: i32,
        flags: u32,
    ) -> *mut Raw_SDL_Renderer;
    fn SDL_SetRenderDrawColor(renderer: *mut Raw_SDL_Renderer, r: u8, g: u8, b: u8, a: u8) -> i32;
    fn SDL_RenderClear(renderer: *mut Raw_SDL_Renderer);
    fn SDL_RenderPresent(renderer: *mut Raw_SDL_Renderer);
    fn SDL_DestoryRenderer(renderer: *mut Raw_SDL_Renderer);
    fn SDL_Delay(ms: u32);
    fn SDL_PollEvent(event: *mut SDL_Event) -> i32;
}

struct SDL_Window {
    raw: *mut Raw_SDL_Window,
}

impl SDL_Window {
    fn create(title: &str, x: i32, y: i32, w: i32, h: i32, flags: u32) -> Result<Self, String> {
        let title = CString::new(title).unwrap();
        let raw = unsafe { SDL_CreateWindow(title.as_ptr(), x, y, w, h, flags) };

        if raw.is_null() {
            Err(format!(
                "Could not initialize SDL: {}",
                unsafe { CStr::from_ptr(SDL_GetError()) }.to_string_lossy()
            ))
        } else {
            Ok(Self { raw })
        }
    }
}

struct SDL_Renderer {
    raw: *mut Raw_SDL_Renderer,
}

fn main() {
    unsafe {
        if SDL_Init(SDL_INIT_VIDEO) < 0 {
            panic!();
        }

        let title = CString::new("Hello from Rust").unwrap();
        let window = SDL_CreateWindow(
            title.as_ptr(),
            0,
            0,
            SCREEN_HEIGHT,
            SCREEN_WIDTH,
            SDL_WINDOW_RESIZABLE,
        );
        if window.is_null() {
            panic!(
                "Could not create SDL Window: {}",
                CStr::from_ptr(SDL_GetError()).to_string_lossy()
            );
        }

        let renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
        if renderer.is_null() {
            panic!(
                "Could not create SDL Renderer: {}",
                CStr::from_ptr(SDL_GetError()).to_string_lossy()
            );
        }

        let mut quit = false;
        while !quit {
            let mut event: SDL_Event = SDL_Event {
                r#type: 0,
                padding: [0; 52],
            };
            while SDL_PollEvent(&mut event as *mut SDL_Event) > 0 {
                if event.r#type == SDL_QUIT {
                    quit = true;
                }
            }

            SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
            SDL_RenderClear(renderer);

            SDL_RenderPresent(renderer);
            SDL_Delay(100);
        }

        SDL_Quit();
    }
}
