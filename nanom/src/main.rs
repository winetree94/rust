use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::Backend,
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Read, Result},
};

fn main() -> Result<()> {
    let filename = "test.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut vec: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal: Terminal<CrosstermBackend<std::io::Stdout>> =
        Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    render(&mut terminal, &mut vec)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn render<B: Backend>(terminal: &mut Terminal<B>, lines: &mut Vec<String>) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            lines.iter().for_each(|line| {
                frame.render_widget(
                    Paragraph::new(format!("{line}"))
                        .white()
                        .on_blue(),
                    area,
                );
            });
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    Ok(())
}
