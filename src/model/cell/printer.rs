use super::Cell;
use crate::input::env::is_debug_mode;
use colored::Colorize;

impl Cell {
    fn _print_in_game(&self) {
        if !self.is_open {
            match self.is_flag {
                false => return print!("{}", " ".on_white()),
                true => return print!("{}", "f".black().on_white()),
            };
        }
        if self.is_mine {
            return print!("{}", "×".red());
        }
        let s: String = match self.neighbor_mine_count {
            0 => String::from("□"),
            _ => format!("{}", self.neighbor_mine_count),
        };
        print!("{}", s);
    }

    pub(in crate::model) fn print_in_result(&self) {
        match self.is_mine {
            true => print!("{}", "×".red()),
            false => {
                let s: String = match self.neighbor_mine_count {
                    0 => String::from("□"),
                    _ => format!("{}", self.neighbor_mine_count),
                };
                let colored = match self.is_open {
                    true => s.white(),
                    false => s.on_blue().white(),
                };
                print!("{}", colored)
            }
        }
    }
    pub(in crate::model) fn print(&self) {
        match is_debug_mode() {
            false => self._print_in_game(),
            true => self.print_in_result(),
        }
    }
}
