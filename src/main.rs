mod domain;
mod application;

fn main() -> Result<(), String> {

    application::display().map_err(|_| "error UX")?;
    Ok(())
}
