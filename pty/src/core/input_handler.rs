use anyhow::Result;
use std::io::Write;

#[derive(Debug)]
pub enum Mode {
    Terminal,
    AI,
}

impl Mode {
    pub fn prefix(&self) -> &str {
        match self {
            Mode::Terminal => "[$]",
            Mode::AI => "[AI]",
        }
    }

    pub fn colored_prompt(&self) -> String {
        match self {
            Mode::Terminal => format!("\x1b[0;32m{}\x1b[0m ", self.prefix()),
            Mode::AI => format!("\x1b[0;35m{}\x1b[0m ", self.prefix()),
        }
    }
}

pub struct InputState {
    mode: Mode,
    line_buffer: String,
    input_buffer: String,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mode: Mode::Terminal,
            line_buffer: String::new(),
            input_buffer: String::new(),
        }
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Terminal => Mode::AI,
            Mode::AI => Mode::Terminal,
        };
    }

    pub fn clear_buffers(&mut self) {
        self.line_buffer.clear();
        self.input_buffer.clear();
    }
}

pub enum InputAction {
    /// Toggle between interaction modes.
    /// eg. terminal, AI
    ToggleMode,
    /// Single character input
    CharacterInput(char),
    LineComplete {
        cmd_text: String,
        raw_line: String,
    },
}

pub struct InputHandler {
    state: InputState,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            state: InputState::new(),
        }
    }

    pub fn state(&self) -> &InputState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut InputState {
        &mut self.state
    }

    pub fn process_stdin(&mut self, buf: &[u8]) -> Result<Vec<InputAction>> {
        const CTRL_SLASH: u8 = 31;

        if buf.len() == 1 && buf[0] == CTRL_SLASH {
            return Ok(vec![InputAction::ToggleMode]);
        }

        let input = String::from_utf8_lossy(buf);
        let mut actions = Vec::new();

        for ch in input.chars() {
            if ch == '\n' || ch == '\r' {
                self.state.line_buffer.push(ch);
                let line = self.state.line_buffer.trim_end();
                let raw_line = self.state.line_buffer.clone();

                let cmd_text = match self.state.mode {
                    Mode::AI => {
                        if line == "/exit" || line == "/quit" {
                            actions.push(InputAction::ToggleMode);
                            self.state.clear_buffers();
                            continue;
                        } else if line.starts_with('/') {
                            line.to_string()
                        } else {
                            format!("/ai {}", line)
                        }
                    }
                    Mode::Terminal => line.to_string(),
                };

                actions.push(InputAction::LineComplete { cmd_text, raw_line });
                self.state.clear_buffers();
            } else {
                actions.push(InputAction::CharacterInput(ch));
                self.state.line_buffer.push(ch);
            }
        }

        Ok(actions)
    }

    pub fn handle_character(
        &mut self,
        ch: char,
        writer: &mut dyn Write,
        stdout: &mut dyn Write,
    ) -> Result<()> {
        match self.state.mode {
            Mode::Terminal => {
                writer.write_all(&[ch as u8])?;
                writer.flush()?;
            }
            Mode::AI => {
                if ch == '\x7f' || ch == '\x08' {
                    if !self.state.input_buffer.is_empty() {
                        self.state.input_buffer.pop();
                        self.state.line_buffer.pop();
                        stdout.write_all(b"\x08 \x08")?;
                        stdout.flush()?;
                    }
                } else {
                    self.state.input_buffer.push(ch);
                    stdout.write_all(&[ch as u8])?;
                    stdout.flush()?;
                }
            }
        }
        Ok(())
    }

    pub fn show_mode_indicator(&self, writer: &mut dyn Write) -> Result<()> {
        let prompt_command = match self.state.mode {
            Mode::Terminal => format!("export PS1='%F{{green}}{}%f '", self.state.mode.prefix()),
            Mode::AI => format!("export PS1='%F{{magenta}}{}%f '", self.state.mode.prefix()),
        };

        writer.write_all(prompt_command.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(b"clear\n")?;
        writer.flush()?;
        Ok(())
    }
}
