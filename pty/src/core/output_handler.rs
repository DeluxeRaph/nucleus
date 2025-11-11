use anyhow::Result;
use std::io::Write;

pub struct OutputHandler {
    stdout: Box<dyn Write + Send>,
}

impl OutputHandler {
    pub fn new(stdout: Box<dyn Write + Send>) -> Self {
        Self { stdout }
    }

    pub fn render_pty_output(&mut self, buf: &[u8]) -> Result<()> {
        self.stdout.write_all(buf)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn render_command_response(&mut self, response: &str) -> Result<()> {
        self.stdout.write_all(b"\r\n")?;
        
        for line in response.lines() {
            self.stdout.write_all(line.as_bytes())?;
            self.stdout.write_all(b"\r\n")?;
        }
        
        self.stdout.flush()?;
        Ok(())
    }

    pub fn render_error(&mut self, error: &str) -> Result<()> {
        let error_msg = format!("Error: {}\r\n", error);
        self.stdout.write_all(error_msg.as_bytes())?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn render_prompt(&mut self, prompt: &str) -> Result<()> {
        self.stdout.write_all(prompt.as_bytes())?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn render_newline(&mut self) -> Result<()> {
        self.stdout.write_all(b"\r\n")?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn get_writer(&mut self) -> &mut dyn Write {
        &mut *self.stdout
    }
}
