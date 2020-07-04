mod app;

fn main() -> anyhow::Result<()> {
    let mut app = app::App::new()?;
    app.run()?;

    Ok(())
}
