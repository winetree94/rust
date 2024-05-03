pub use todo::app::TodoApp;

fn main() -> Result<(), std::io::Error> {
    let mut app = TodoApp::new();
    app.start()?;
    Ok(())
}
