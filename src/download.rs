use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use lofty::{
    config::WriteOptions,
    file::TaggedFileExt,
    picture::{Picture, PictureType},
    prelude::*,
    tag::Tag,
};
use std::fmt::Write;
use std::io::{Read, Write as IoWrite};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::Cursor;
use std::{fs::File, io};

fn watch_pause() -> Arc<AtomicBool> {
    let paused = Arc::new(AtomicBool::new(false));
    let flag = paused.clone();
    thread::spawn(move || {
        let mut buf = [0u8; 1];
        loop {
            match io::stdin().read(&mut buf) {
                Ok(0) | Err(_) => return,
                Ok(_) => flag.store(!flag.load(Ordering::SeqCst), Ordering::SeqCst),
            }
        }
    });
    paused
}

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

    println!("press Enter to pause or resume this download");
    let paused = watch_pause();
    let mut reader = downer.as_reader();
    let mut out = File::create(pth).expect("there was an error creating the file");
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        while paused.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(100));
        }
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        out.write_all(&buf[..n])?;
        progress.inc(n as u64);
    }
    out.flush()?;
    progress.finish();
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