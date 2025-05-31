#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]     //repr tells the compiler how to layout the memory here its u8
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]        //transparent- applies to structs with exactly one field here its u8 making it safe to treat color as u8
struct ColorCode(u8);

impl ColorCode{
    fn new(foreground: Color, background: Color) -> ColorCode{
        ColorCode((background as u8)<<4 | (foreground as u8))       //background is stored in the first 4 bits and foreground in the later and '|' merges them
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_character: u8,        //ascii code of character
    color_code: ColorCode,      //foreground and background colors
}

//global variables for buffer height and width
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;
#[repr(transparent)]
struct Buffer{
    // A 2D array of screen characters.
    //each entry holds a single character and its color code.
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


//A Writer writes characters to the VGA text buffer
pub struct Writer{
    column_position: usize,     //current position in the row
    color_code: ColorCode,      //current text color
    buffer: &'static mut Buffer,        //reference to the VGA text in buffer memory where the characters are displayed
    //'static means that it lives for the whole duration of the program
}

impl Writer{
    //writes a string to vga buffer
    //printable ascii characters 0x20 - 0x7e and newline ('\n') are written as it
    //any other byte is replaced with 0xfe a solid block placeholder

    pub fn write_string(&mut self, s: &str){
        for byte in s.bytes(){     //s.bytes() convert it into raw bytes (u8)
            match byte{
                //printable ascii characters or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                //non printable characters
                _=> self.write_byte(0xfe),
            }
        }
    }
    //writes a single byte to the screen.
    pub fn write_byte(&mut self, byte: u8){
        
        match byte{
        
        b'\n' => self.new_line(),       //if its a new line byte then new_line function is called

        byte=>{
            if self.column_position >= BUFFER_WIDTH{
                self.new_line();
            }

            let row=BUFFER_HEIGHT-1;
            let col=self.column_position;

            let color_code=self.color_code;
            
            //write the character to screen buffer

            self.buffer.chars[row][col].write(ScreenChar{
                ascii_character: byte,
                color_code,
            });

            //advance the columnd cursor
            self.column_position+=1;
            }
        }
    }

    fn new_line(&mut self){/*TODO*/}
}

use core::fmt;

impl fmt::Write for Writer{
    fn write_str(&mut self, s:&str)-> fmt::Result{
        self.write_string(s);
        Ok(())
    }
}


pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe{ &mut *(0xb8000 as mut* Buffer)},
    }

    writer.write_byte(b'H');
    writer.write_string("ello! ");
    write!(writer, "the numbers are {} and {}", 42, 1.0/3.0).unwrap()
}
