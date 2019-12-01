extern crate bear_lib_terminal;

use bear_lib_terminal::{
    Color,
    geometry::{ Rect, Size },
    terminal::{
        self,
        config,
        Alignment, HAlign, VAlign,
        Event, KeyCode,
    },
};

const INIT_TERM_W: i32 = 80;
const INIT_TERM_H: i32 = 30;

const PADDING_X: i32 = 4;
const PADDING_Y: i32 = 2;
// additional padding
const PADDING_TOP: i32 = 2;

fn main() {
    let lorem_ipsum =
       "[c=orange]Lorem[/c] ipsum dolor sit amet, consectetur adipisicing elit, \
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
        [c=orange]Ut[/c] enim ad minim veniam, quis nostrud exercitation ullamco \
        laboris nisi ut aliquip ex ea commodo consequat. [c=orange]Duis[/c] aute \
        irure dolor in reprehenderit in voluptate velit esse cillum dolore eu \
        fugiat nulla pariatur. [c=orange]Excepteur[/c] sint occaecat cupidatat \
        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    
    terminal::open("Text Alignment Demo", INIT_TERM_W as u32, INIT_TERM_H as u32);
    terminal::set(config::Window::empty().resizeable(true));
    terminal::composition(true);
    
    let mut frame = resize_frame(Size::new(INIT_TERM_W, INIT_TERM_H));
    
    let mut h_align = HAlign::Left;
    let mut v_align = VAlign::Top;
    
    loop {
        terminal::clear(None);
        
        terminal::with_background(Color::from_rgb(32, 32, 32), || {
            terminal::clear(Some(frame.clone()));
        });
        
        terminal::print_xy(PADDING_X, 1, "Use the arrow keys to change alignment.");
        terminal::print_xy(PADDING_X, 2, &format!("[color=dark turquoise]Current alignment:[/color] [color=turquoise]H:{:?}, V:{:?}[/color]", h_align, v_align));
        terminal::print_ext(frame, Alignment::new(h_align.clone(), v_align.clone()), lorem_ipsum);
        
        terminal::refresh();
        
        if let Some(e) = terminal::wait_event() {
            use {HAlign::*, VAlign::*,};
            match e {
                Event::KeyPressed{ key: KeyCode::Escape, ..} | Event::Close => break,
                Event::Resize{width, height} => frame = resize_frame(Size::new(width, height)),
                Event::KeyPressed{ key: KeyCode::Left, ..} => h_align = match h_align {
                    Left => Left,     Center => Left,     Right => Center,
                },
                Event::KeyPressed{ key: KeyCode::Right, ..} => h_align = match h_align {
                    Left => Center,   Center => Right,    Right => Right,
                },
                Event::KeyPressed{ key: KeyCode::Up, ..} => v_align = match v_align {
                    Top => Top,       Middle => Top,      Bottom => Middle,
                },
                Event::KeyPressed{ key: KeyCode::Down, ..} => v_align = match v_align {
                    Top => Middle,    Middle => Bottom,   Bottom => Bottom,
                },
                _ => {}
            }
        }
    }
    
    terminal::close();
}

fn resize_frame(term_size: Size) -> Rect {
    Rect::from_values(
        PADDING_X,
        PADDING_Y + PADDING_TOP,
        term_size.width - PADDING_X * 2,
        term_size.height - PADDING_Y * 2 - PADDING_TOP,
    )
}