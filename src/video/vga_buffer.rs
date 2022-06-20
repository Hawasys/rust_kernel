use io::{inbyte, outbyte};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorForeground {
    Black = 0x00,
	Blue = 0x01,
	Green = 0x02,
	Cyan = 0x03,
	Red = 0x04,
	Magenta = 0x05,
	Brown = 0x06,
	Grey = 0x07,
	DarkGray = 0x08,
	DarkGreen = 0x09,
	LightGreen = 0x0a,
	LightCyan = 0x0b,
	LightRed = 0x0c,
	LightMagenta = 0x0d,
	LightBrown = 0x0e,
	White = 0x0f,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorBackground{
    Black = 0x00,
    Blue = 0x10,
    Green = 0x20,
    Cyan = 0x30,
    Red = 0x40,
    Magenta = 0x50,
    Brown = 0x60,
    LightGray = 0x70,
    BlinkingBlack = 0x80,
    BlinkingBlue = 0x90,
    BlinkingGreen = 0xA0,
    BlinkingCyan = 0xB0,
    BlinkingRed = 0xC0,
    BlinkingMagenta = 0xD0,
    BlinkingYellow = 0xE0,
    BlinkingWhite = 0xF0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: ColorForeground, background: ColorBackground) -> ColorCode {
        ColorCode((foreground as u8) | (background as u8))
    }
}

const BUFFER_HEIGHT: u8 = 25;
const BUFFER_WIDTH: u16 = 80;
const VGA_BUFFER_START: u32 = 0xb8000 as u32;

pub unsafe fn move_cursor(pos:u16)
{
	outbyte(0x3D4, 0x0F);
	outbyte(0x3D5, (pos & 0xFF) as u8);
	outbyte(0x3D4, 0x0E);
	outbyte(0x3D5, (pos >> 8) as u8 & 0xFF);
}

pub unsafe fn move_cursor_xy(x:u16, y:u16){
    let pos: u16 = y * BUFFER_WIDTH + x;
 
	outbyte(0x3D4, 0x0F);
	outbyte(0x3D5, (pos & 0xFF) as u8);
	outbyte(0x3D4, 0x0E);
	outbyte(0x3D5, (pos >> 8) as u8 & 0xFF);
}

pub unsafe fn get_cursor_pos() -> u16 {
    let mut pos: u16 = 0;
    outbyte(0x3D4, 0x0F);
    pos |= inbyte(0x3D5) as u16;
    outbyte(0x3D4, 0x0E);
    pos |= (inbyte(0x3D5) as u16) << 8;
    return pos;
}

pub unsafe fn write_char(character: &char, color: ColorCode){
    let pos:u16 = get_cursor_pos();
    let vga_pos = VGA_BUFFER_START + 2*(pos as u32);
    *(vga_pos as *mut char) = *character;
    let color_buffer = vga_pos+1;
    *(color_buffer as *mut ColorCode) = color;
    move_cursor(pos+1);
}

pub unsafe fn write_string(string: &str, color: ColorCode){
    let mut pos:u16 = get_cursor_pos();
    let bytes = string.as_bytes();
    
    //why 
    assert!(bytes.is_empty());
    

    write_char(&'S', color);

    for c in bytes{
        let vga_pos = VGA_BUFFER_START + 2*(pos as u32);
        *(vga_pos as *mut u8) = *c;
        let color_buffer = vga_pos+1;
        *(color_buffer as *mut ColorCode) = color;
        pos += 1;
    }
    move_cursor(pos+1);

  
}
