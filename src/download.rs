use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::{fs::File, io};
pub fn download(link: String, pth: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut downer = ureq::get(link)
        .call()
        .expect("There was an error downloading your file")
        .into_body();
    let progress = ProgressBar::new(downer.content_length().unwrap_or(0));
    progress.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));

    io::copy(
        &mut progress.wrap_read(downer.as_reader()),
        &mut File::create(pth).expect("there was an error creating the file"),
    )
    .expect("There was an error");
    Ok(())
}
