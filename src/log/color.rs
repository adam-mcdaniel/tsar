extern crate crossterm;

use crossterm::{
    Color::{Blue, Cyan, Green, Magenta, Red, Reset, White, Yellow},
    Colored,
    Colored::Fg,
};
use std::fmt::Display;

use crate::log::LazyPrint;

pub fn white(s: impl Display) -> LazyPrint<Colored, impl Display, Colored> {
    LazyPrint::new(Fg(White), s, Fg(Reset))
}

pub fn yellow(s: impl Display) -> LazyPrint<Colored, impl Display, Colored> {
    LazyPrint::new(Fg(Yellow), s, Fg(Reset))
}

pub fn red(s: impl Display) -> LazyPrint<Colored, impl Display, Colored> {
    LazyPrint::new(Fg(Red), s, Fg(Reset))
}

pub fn green(s: impl Display) -> LazyPrint<Colored, impl Display, Colored> {
    LazyPrint::new(Fg(Green), s, Fg(Reset))
}

pub fn blue(s: impl Display) -> LazyPrint<Colored, impl Display, Colored> {
    LazyPrint::new(Fg(Blue), s, Fg(Reset))
}

pub fn magenta(s: impl Display) -> LazyPrint<Colored, impl Display, Colored> {
    LazyPrint::new(Fg(Magenta), s, Fg(Reset))
}

pub fn cyan(s: impl Display) -> LazyPrint<Colored, impl Display, Colored> {
    LazyPrint::new(Fg(Cyan), s, Fg(Reset))
}
