use std::io::Result;

#[cfg(windows)] use winres::WindowsResource;

fn main() -> Result<()> {
    #[cfg(windows)] {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("assets/app-icon.ico")
            .compile()?;
    }
    Ok(())
}