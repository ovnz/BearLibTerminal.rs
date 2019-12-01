use bear_lib_terminal_sys as ffi;

#[derive(Clone, PartialEq, Debug)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone, PartialEq, Debug)]
pub enum VAlign {
    Top,
    Middle,
    Bottom,
}

pub struct Alignment{
    h: HAlign,
    v: VAlign,
}
impl Alignment {
    pub fn new(align_h: HAlign, align_v: VAlign) -> Self {
        Alignment {
            h: align_h,
            v: align_v,
        }
    }
}
impl Default for Alignment {
    fn default() -> Self {
        Alignment {
            h: HAlign::Left,
            v: VAlign::Top,
        }
    }
}
impl From<Alignment> for i32 {
    fn from(align: Alignment) -> Self {
        let h = match align.h {
            HAlign::Left =>     ffi::TK_ALIGN_LEFT,
            HAlign::Center =>   ffi::TK_ALIGN_CENTER,
            HAlign::Right =>    ffi::TK_ALIGN_RIGHT,
        };
        let v = match align.v {
            VAlign::Top =>      ffi::TK_ALIGN_TOP,
            VAlign::Middle =>   ffi::TK_ALIGN_MIDDLE,
            VAlign::Bottom =>   ffi::TK_ALIGN_BOTTOM,
        };
        h | v
    }
}