use std::{fmt,env,io::{IsTerminal,self},sync::LazyLock,error::Error,fmt::Write,
    borrow::Cow,
};

#[derive(Default, Debug, Clone, PartialEq)] 
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    #[default]
    Notset,
    Unset, // inherited
    Gray(u8),
    Palette216(u8),
    True(u8,u8,u8)
}
#[derive(Clone)] //Debug, 
pub struct ColorHolder<B>{
    inner: B,
    fg: Color,
    bg: Color,
    bright: bool,
    bright_bg: bool,
    bold: bool,
    italic: bool,
    blink: bool,
    underline: bool,
    hidden: bool,
    reversed: bool,
    strikethrough: bool,
    dimmed: bool,
}
pub trait Colorized : Sized {
    fn color(self, fg: Color) -> ColorHolder<Self> {
        ColorHolder {
            inner: self,
            fg,
            bg: Default::default(),
            bright: Default::default(),
            bright_bg: Default::default(),
            bold: Default::default(),
            italic: Default::default(),
            blink: Default::default(),
            underline: Default::default(),
            hidden: Default::default(),
            reversed: Default::default(),
            strikethrough: Default::default(),
            dimmed: Default::default(),
        }
    }
    fn attribute(self) -> ColorHolder<Self> {
        ColorHolder {
            inner: self,fg:Default::default(),
            bg: Default::default(),
            bright: Default::default(),
            bright_bg: Default::default(),
            bold: Default::default(),
            italic: Default::default(),
            blink: Default::default(),
            underline: Default::default(),
            hidden: Default::default(),
            reversed: Default::default(),
            strikethrough: Default::default(),
            dimmed: Default::default(),
        }
    }
    fn on(self) -> ColorHolder<Self> {
        self.color(Color::Unset)
    }
    fn blue(self) -> ColorHolder<Self> {
          self.color(Color::Blue)
    }
    fn black(self) -> ColorHolder<Self> {
        self.color(Color::Black)
    }
    fn yellow(self) -> ColorHolder<Self> {
          self.color(Color::Yellow)
    }
    fn red(self) -> ColorHolder<Self> {
          self.color(Color::Red)
    }
    fn magenta(self) -> ColorHolder<Self> {
          self.color(Color::Magenta)
    }
    fn white(self) -> ColorHolder<Self> {
          self.color(Color::White)
    }
    fn green(self) -> ColorHolder<Self> {
        self.color(Color::Green)
    }
    fn cyan(self) -> ColorHolder<Self> {
          self.color(Color::Cyan)
    }
    fn default(self) -> ColorHolder<Self> {
          self.color(Color::default())
    }
    fn gray(self, gray:u8) -> ColorHolder<Self> {
        if gray < 24 {
            self.color(Color::Gray(gray+232))
        } else {
            self.attribute()
        }
    }
    fn color_num(self, num:u8) -> ColorHolder<Self> {
        if (16..232).contains(&num) {
            self.color(Color::Palette216(num))
        } else {
            self.attribute()
        }
    }
    fn rgb(self, r:u8, g:u8, b:u8) -> ColorHolder<Self> {
        self.color(Color::True(r,g,b))
    }
    fn rgb_color(self, color:&(u8, u8, u8)) -> ColorHolder<Self> {
        self.color(Color::True(color.0,color.1,color.2))
    }
    fn blink(self) -> ColorHolder<Self> {
          let mut res = self.attribute();
          res.blink = true;
          res
    }
    fn blink_fast(self) -> ColorHolder<Self> {
          self.blink()
    }
    fn hidden(self) -> ColorHolder<Self> {
          let mut res = self.attribute();
          res.hidden = true;
          res
    }
    fn strikethrough(self) -> ColorHolder<Self> {
          let mut res = self.attribute();
          res.strikethrough = true;
          res
    }
    fn italic(self) -> ColorHolder<Self> {
          let mut res = self.attribute();
          res.italic = true;
          res
    }
    fn bold(self) -> ColorHolder<Self> {
          let mut res = self.attribute();
          res.bold = true;
          res
    }
    fn underline(self) -> ColorHolder<Self> {
          let mut res = self.attribute();
          res.underline = true;
          res
    }
    fn bright(self) -> ColorHolder<Self> {
          let mut res = self.attribute();
          res.bright = true;
          res
    }
    fn reversed(self) -> ColorHolder<Self> {
          let mut res = self.attribute();
          res.reversed = true;
          res
    }
    fn dimmed(self) -> ColorHolder<Self> {
        let mut res = self.attribute();
          res.dimmed = true;
          res
    }
}

impl<T> ColorHolder<T> {
    fn color(mut self, color: Color) -> Self {
        if self.fg != Color::Notset {
            self.bg = color
        } else {
            self.fg = color
        }
        self
    }
    /// Change the current fore/back color to the blue
    ///
    /// Consequent this call will update the color to the last value
    ///
    /// let blue = "blue".blue();
    pub fn blue(self) -> Self {
        self.color(Color::Blue)
    }
    pub fn black(self) -> Self {
        self.color(Color::Black)
    }
    pub fn yellow(self) -> Self {
        self.color(Color::Yellow)
    }
    pub fn white(self) -> Self {
        self.color(Color::White)
    }
    pub fn magenta(self) -> Self {
        self.color(Color::Magenta)
    }
    pub fn cyan(self) -> Self {
        self.color(Color::Cyan)
    }
    pub fn red(self) -> Self {
        self.color(Color::Red)
    }
    pub fn green(self) -> Self {
        self.color(Color::Green)
    }
    /// Change the current fore/back color to a shade of gray specified by the parameter
    ///
    /// 24 grades from 0 to 23 are available
    ///
    /// Consequent this call will update the color to the last value
    ///
    /// let gray = "gray".gray(10);
    pub fn gray(self, shade: u8) -> Self {
        if shade < 24 {
            self.color(Color::Gray(shade+232))
        } else {
            self
        }
    }
    /// Change the current fore/back color to a color from the 216 colors palette
    ///
    /// Consequent this call will update the color to the last value
    ///
    /// let orange = "orange".color_num(214);
    pub fn color_num(self, num: u8) -> Self {
        if (16..232).contains(&num) {
            self.color(Color::Palette216(num))
        } else {
            self
        }
    }
    /// Change the current fore/back color to a color based on RGB value
    ///
    /// Consequent this call will update the color to the last value
    ///
    /// let greenish = "greenish pale".rgb(108, 140, 107);
    pub fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.color(Color::True(r,g,b))
    }
    pub fn rgb_color(self, color:&(u8, u8, u8)) -> Self {
        self.color(Color::True(color.0,color.1,color.2))
    }
    pub fn bright(mut self) -> Self {
        if self.bg != Color::Notset {
            self.bright_bg = true
        } else { //if self.bg != Color::Notset {
            self.bright = true
        }
        self
    }
    pub fn on(mut self) -> Self {
        if self.fg == Color::Notset {
            self.fg = Color::Unset
        }
        self.bg = Color::Unset;
        self
    }
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn reversed(mut self) -> Self {
        self.reversed = true;
        self
    }
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
    pub fn blink(mut self) -> Self {
        self.blink = true;
        self
    }
    pub fn dimmed(mut self) -> Self {
        self.dimmed = true;
        self
    }

    fn ansi(&self) -> String {
        let mut color = String::new();
        // TODO consolidate as set_color(foreground:bool, color: Color)
        if self.fg != Color::Notset && self.fg != Color::Unset {
            match self.fg {
                Color::Gray(gray) => {
                    let _ = write!(color, "38;5;{gray}");
                }
                Color::Palette216(palette) => write!{color, "38;5;{palette}"}.unwrap(),
                Color::True(r,g,b) => write!{color, "38;2;{r};{g};{b}"}.unwrap(),
               _ => { if self.bright {color.push('9')} else {color.push('3')} 
                color .push(get_color_num(&self.fg)) }
            }
        }
        if self.bg != Color::Notset && self.bg != Color::Unset {
            if !color.is_empty() {
                color.push(';')
            }
            match self.bg {
                Color::Gray(gray) => {
                    let _ = write!(color, "48;5;{gray}");
                }
                Color::Palette216(palette) => write!{color, "48;5;{palette}"}.unwrap(),
                Color::True(r,g,b) => write!{color, "48;2;{r};{g};{b}"}.unwrap(),
               _ => { 
                   if self.bright_bg {color.push_str("10")} else {color.push('4')} 
                   color .push(get_color_num(&self.bg))
               }
            }
        }
        if self.underline { if !color.is_empty() {
            color.push(';')
        }color.push('4'); }
        if self.bold { if !color.is_empty() {
            color.push(';')
        }color.push('1')}
        if self.italic { if !color.is_empty() {
            color.push(';')
        }color.push('3')}
        if self.blink { if !color.is_empty() {
            color.push(';')
        }color.push('5')}
        if self.strikethrough { if !color.is_empty() {
            color.push(';')
        }color.push('9')}
        if self.reversed { if !color.is_empty() {
            color.push(';')
        }color.push('7')}
        if self.hidden { if !color.is_empty() {
            color.push(';')
        }color.push('8')}
        if self.dimmed { if !color.is_empty() {
            color.push(';')
        }color.push('2')}
        color
    }
    #[cfg(partial_reset)]
    fn ansi_clear(&self) -> String {
        let mut clear = String::new();
        if self.underline { 
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear.push('2'); clear.push('4')
        }
        if self.bold { 
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear.push('2'); clear.push('2')
        }
        if self.italic {
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear.push('2'); clear.push('3')
        }
        if self.blink {
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear.push('2'); clear.push('5')
        } 
        if self.strikethrough {
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear.push('2'); clear.push('9')
        }
        if self.reversed {
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear.push('2'); clear.push('7')
        } 
        if self.hidden {
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear.push('2'); clear.push('8')
        } 
        if self.dimmed {
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear.push('2'); clear.push('2')
        }
        if self.fg != Color::Notset && self.fg != Color::Unset {
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear .push('3');
            clear .push(get_color_num(&Color::Unset))
        }
        if self.bg != Color::Notset && self.bg != Color::Unset {
            if !clear.is_empty() {
                clear.push(';') 
            }
            clear .push('4');
            clear .push(get_color_num(&Color::Unset))
        }
        clear
    }
}
fn get_color_num(color: &Color) -> char {
    match color {
        Color::Black => '0',
        Color::Red => '1',
        Color::Green  => '2',
        Color::Yellow => '3',
        Color::Blue => '4',
        Color::Magenta => '5',
        Color::Cyan => '6',
        Color::White => '7',
        _ => '9'
    }
}
impl<C> fmt::Display for ColorHolder<C> 
where
    C: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = self.ansi();
        if !*ENABLE_COLOR || color.is_empty() {
            <C>::fmt(&self.inner, f)
        } else {
            let mut fmt_str = format!("{}", self.inner);
            while let Some(stripped_str) = fmt_str.strip_suffix("\x1b[0m") {
                fmt_str = stripped_str.to_string()
            }
            let mut current = 0;
            while let Some(offset) = find_after(&fmt_str, "\x1b[0m", current) {
                fmt_str.replace_range(offset..4+offset, &format!("\x1b[0m\x1b[{color}m"));
                current = offset+4 // "\x1b[0m".len()
            }
            #[cfg(not(partial_reset))]
            {write!(f, "\x1b[{color}m{fmt_str}\x1b[0m")}
            #[cfg(partial_reset)]
            {
                f.write_str(&format!("\x1b[{color}m"))?;
                <C>::fmt(&self.inner, f)?;
                f.write_str(&format!("\x1b[{}m", self.ansi_clear()))
            }
        }
    }
}
fn find_after(s: &str, sub: &str, after: usize) -> Option<usize> {
    if after == 0 {
        s.find(sub)
    } else {
        s[after..].find(sub).map(|i| i + after)
    }
}
impl<S: fmt::Display> fmt::Debug for ColorHolder<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl<T: std::fmt::LowerHex> fmt::LowerHex for ColorHolder<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        if *ENABLE_COLOR {
            let color = self.ansi();
            if !color.is_empty() {
                f.write_str(&format!("\x1b[{color}m"))?
            }
        }
        fmt::LowerHex::fmt(&self.inner, f)?;
        if *ENABLE_COLOR  {
            f.write_str("\x1b[0m")?
        }
        Ok(())
    }
}
impl<T: std::fmt::Octal> fmt::Octal for ColorHolder<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        if *ENABLE_COLOR {
            let color = self.ansi();
            if !color.is_empty() {
                f.write_str(&format!("\x1b[{color}m"))?
            }
        }
        fmt::Octal::fmt(&self.inner, f)?;
        if *ENABLE_COLOR  {
            f.write_str("\x1b[0m")?
        }
        Ok(())
    }
}
//
impl Colorized for &str {}
impl Colorized for String {}
impl Colorized for u32 {}
impl Colorized for u64 {}
impl Colorized for usize {}
impl Colorized for i32 {}
impl Colorized for Cow<'_, String> {}
impl<S: std::fmt::Display + std::fmt::Debug> Error for ColorHolder<S> {}
pub static ENABLE_COLOR: LazyLock<bool> = LazyLock::new(from_env);

fn from_env() -> bool {
    (env::var("CLICOLOR").map(|val| val == "true").unwrap_or(false)
        || env::var("COLORTERM").map(|_val| true).unwrap_or(false)
        || io::stdout().is_terminal()
        || env::var("TERM").map(|val| val.contains("color")).unwrap_or(false))
        && (!env::var("NO_COLOR").map(|val| val == "true").unwrap_or(false) 
        || env::var("CLICOLOR_FORCE").map(|val| val != "false").unwrap_or(false)) 
}