use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use lofty::{
    config::WriteOptions,
    file::TaggedFileExt,
    picture::{Picture, PictureType},
    prelude::*,
    tag::Tag,
};
use std::fmt::Write;
use std::io::Cursor;
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

pub fn embed_cover(path: String, cover_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let data = ureq::get(cover_url).call()?.body_mut().read_to_vec()?;
    let mut pic = Picture::from_reader(&mut Cursor::new(data))?;
    pic.set_pic_type(PictureType::CoverFront);

    let mut file = lofty::read_from_path(path.clone())?;
    if file.primary_tag().is_none() {
        let ty = file.primary_tag_type();
        file.insert_tag(Tag::new(ty));
    }
    let tag = file.primary_tag_mut().unwrap();
    tag.push_picture(pic);
    tag.save_to_path(path, WriteOptions::default())?;
    Ok(())
}
