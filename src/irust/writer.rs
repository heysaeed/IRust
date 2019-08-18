use crossterm::{ClearType, Color};

use crate::irust::{IRust, IRustError};

impl IRust {
    pub fn write(&mut self, out: &str, color: Color) -> Result<(), IRustError> {
        self.color.set_fg(color)?;
        for c in out.chars() {
            self.terminal.write(c)?;
            self.cursor.move_right_unbounded();
        }
        self.color.reset()?;
        Ok(())
    }

    pub fn write_str_at(&mut self, s: &str, x: usize, y: usize) -> Result<(), IRustError> {
        self.cursor.goto(x, y);
        self.terminal.write(s)?;
        Ok(())
    }

    pub fn write_from_terminal_start(&mut self, out: &str, color: Color) -> Result<(), IRustError> {
        self.cursor.goto(0, self.cursor.pos.current_pos.1);
        self.write(out, color)?;
        Ok(())
    }

    pub fn write_newline(&mut self) -> Result<(), IRustError> {
        self.move_screen_cursor_to_last_line();

        // check for scroll
        if self.cursor.is_at_last_terminal_row() {
            self.scroll_up(1);
        }
        self.cursor.move_down(1);
        self.cursor.use_current_row_as_starting_row();

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), IRustError> {
        self.terminal.clear(ClearType::All)?;
        self.buffer.goto_start();
        self.cursor.pos.starting_pos = (0, 0);
        self.cursor.goto(4, 0);
        self.write_input()?;
        Ok(())
    }

    pub fn scroll_up(&mut self, n: usize) {
        let _ = self.terminal.scroll_up(n as i16);
        self.cursor.move_up(n as u16);
        self.cursor.pos.starting_pos.1 = self.cursor.pos.starting_pos.1.saturating_sub(n);
    }
}
