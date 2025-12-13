use std::{fmt,env,io::{IsTerminal,self},sync::LazyLock,error::Error};

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
    Unset,
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
        if self.fg != Color::Notset && self.fg != Color::Unset {
            if self.bright {color.push('9')} else {color.push('3')} 
            color .push(get_color_num(&self.fg))
        }
        if self.bg != Color::Notset && self.bg != Color::Unset {
            if self.bright_bg {color.push_str("10")} else {color.push('4')} 
            color .push(get_color_num(&self.bg))
        }
        if !color.is_empty() {
            color.push(';')
        }
        if self.underline { color.push('4'); color.push(';')}
        if self.bold { color.push('1'); color.push(';')}
        if self.italic { color.push('3'); color.push(';')}
        if self.blink { color.push('5'); color.push(';')}
        if self.strikethrough { color.push('9'); color.push(';')}
        if self.reversed { color.push('7'); color.push(';')}
        if self.hidden { color.push('8'); color.push(';')}
        if self.dimmed { color.push('2'); color.push(';')}
        color
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
        _ => '0'
    }
}
impl<C> fmt::Display for ColorHolder<C> 
where
    C: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = self.ansi();
        if !*ENABLE_COLOR || color.is_empty() {
            write!(f, "{}", self.inner)
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
            write!(f, "\x1b[{color}m{fmt_str}\x1b[0m")//, self.inner)
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
impl fmt::LowerHex for ColorHolder<u32> {
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
impl Colorized for &str {}
impl Colorized for String {}
impl Colorized for u32 {}
impl<S: std::fmt::Display + std::fmt::Debug> Error for ColorHolder<S> {}
pub static ENABLE_COLOR: LazyLock<bool> = LazyLock::new(from_env);

fn from_env() -> bool {
     (env::var("CLICOLOR").map(|val| val == "true").unwrap_or(false)
        || io::stdout().is_terminal()
        || env::var("TERM").map(|val| val.contains("color")).unwrap_or(false))
        && (!env::var("NO_COLOR").map(|val| val == "true").unwrap_or(false) 
        || env::var("CLICOLOR_FORCE").map(|val| val != "false").unwrap_or(false)) 
}