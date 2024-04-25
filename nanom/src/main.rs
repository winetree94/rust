use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::Backend, layout::Rect, prelude::{CrosstermBackend, Stylize, Terminal}, widgets::{Block, Borders, Paragraph}
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

fn startup() -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    Ok(())
}

fn render<B: Backend>(terminal: &mut Terminal<B>, lines: &mut Vec<String>) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            lines.iter().enumerate().for_each(|(index, line)| {
                let area = frame.size();
                // let area = Rect::new(0, index as u16 * 2, frame.size().width, 1);

                frame.render_widget(
                    Paragraph::new(format!("{line}")).block(Block::new().borders(Borders::ALL)),
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
