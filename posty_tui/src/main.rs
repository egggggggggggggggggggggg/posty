use posty_tui::app::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new("Test".to_string());
    ratatui::run(|terminal| app.run(terminal))
}
