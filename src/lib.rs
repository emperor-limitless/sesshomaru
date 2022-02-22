#![allow(unused)]
#![allow(non_upper_case_globals)]
extern crate sdl2;
use once_cell::sync::OnceCell;
use std::clone::*;
use sdl2::event::Event;
pub use sdl2::keyboard::*;
use sdl2::video::Window;
use sdl2::video::*;
use std::collections::HashMap;
use std::sync::Mutex;

//static sdl_context: OnceCell<Mutex<sdl2::Sdl>> = OnceCell::new();
static mut video_subsystem: OnceCell<Mutex<sdl2::VideoSubsystem>> = OnceCell::new();
static mut window: OnceCell<Mutex<sdl2::video::Window>> = OnceCell::new();
static mut events: OnceCell<Mutex<sdl2::EventPump>> = OnceCell::new();
static mut keys: OnceCell<Mutex<HashMap<Scancode, bool>>> = OnceCell::new();
static mut old_keys: OnceCell<Mutex<HashMap<Scancode, bool>>> = OnceCell::new();
static mut pressed_keys: OnceCell<Mutex<HashMap<Scancode, bool>>> = OnceCell::new();

fn get_event() -> &'static mut sdl2::EventPump {
    unsafe { events.get_mut().unwrap().get_mut().unwrap() }
}
pub fn key_pressed(key: Scancode) -> bool {
    unsafe { 
let k = pressed_keys.get_mut().unwrap().get_mut().unwrap();
if !k.contains_key(&key) { return false; }
k[&key] }
}
pub fn key_held(key: Scancode) -> bool {
    unsafe {
let k = keys.get_mut().unwrap().get_mut().unwrap();
    if !k.contains_key(&key) { return false; }
k[&key] }
}

pub fn init(title: &str, screen_width: u32, screen_height: u32) {
    unsafe {
        let mut sdl_context = sdl2::init().unwrap();
        video_subsystem.set(Mutex::new(sdl_context.video().unwrap()));
        window.set(Mutex::new(
            video_subsystem
                .get_mut()
                .unwrap()
                .get_mut()
                .unwrap()
                .window(title, screen_width, screen_height)
                .position_centered()
                .build()
                .unwrap(),
        ));
        events.set(Mutex::new(sdl_context.event_pump().unwrap()));
        keys.set(Mutex::new(HashMap::new()));
        old_keys.set(Mutex::new(HashMap::new()));
        pressed_keys.set(Mutex::new(HashMap::new()));
    }
}
pub fn poll() -> bool {
    unsafe {
        pressed_keys.get_mut().unwrap().get_mut().unwrap().clear();
        for event in events.get_mut().unwrap().get_mut().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return false;
                }
                Event::KeyDown {
                    scancode, repeat, ..
                } => {
                    let key = scancode.unwrap();
                    let k = pressed_keys.get_mut().unwrap().get_mut().unwrap();
                        let k2 = keys.get_mut().unwrap().get_mut().unwrap();
                    if !repeat && !k.contains_key(&key) {
                        k.insert(key, true);
                    } if !k2.contains_key(&key) {
                        if !k2.contains_key(&key) {
                            k2.insert(key, true);
                        }
                    }
                }
                Event::KeyUp { scancode, .. } => {
                    let key = scancode.unwrap();
                    let k = keys.get_mut().unwrap().get_mut().unwrap();
                    if k.contains_key(&key) {
                        k.remove(&key);
                    }
                }
                _ => {}
            }
        }
    }
    return true;
}