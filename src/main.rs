use std::{
    ffi::OsStr,
    fs::{self},
    path::PathBuf,
};

use anyhow::bail;
use id3::{ErrorKind, Tag, TagLike};

fn process_album(artist: &OsStr, path: &PathBuf) -> anyhow::Result<()> {
    let album = path.file_name().expect("has basename");
    let tracks: Vec<_> = fs::read_dir(path)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|d| d.path())
        .filter(|p| p.is_file())
        .collect();

    let album_length = tracks.len() as u32;
    for track_path in tracks {
        let mut tag = match Tag::read_from_path(&track_path) {
            Ok(tag) => tag,
            Err(id3::Error {
                kind: ErrorKind::NoTag,
                ..
            }) => Tag::new(),
            Err(err) => bail!(err),
        };
        let artist = artist.to_string_lossy();
        let album = album.to_string_lossy();
        let track = track_path.file_name().expect("has basename");
        tag.set_artist(artist.as_ref());
        tag.set_album_artist(artist.as_ref());
        tag.set_album(album.as_ref());

        // When we split chapters from youtube we create titles that
        // look like "NUM - TITLE"
        let title_track = track.to_string_lossy();
        let (track, title) = match title_track.split_once('-') {
            Some(elm) => (elm.0.trim(), elm.1.trim()),
            None => bail!("track is not in expected format"),
        };
        tag.set_title(title);
        tag.set_track(track.parse()?);
        tag.set_total_tracks(album_length);

        // Do the actual write of id3 tag info
        tag.write_to_path(&track_path, id3::Version::Id3v24)?;
        println!("Wrote tag: {track} - {title}");
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => bail!("Need to pass a path to an Artist"),
    };

    let full_path = fs::canonicalize(path)?;
    if !full_path.is_dir() {
        bail!("program operates on Artist/Album/Track");
    }

    // Grab the artist
    let artist = full_path.file_name().expect("has basename");

    // Find Albums
    let albums: Vec<_> = fs::read_dir(&full_path)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|d| d.path())
        .filter(|p| p.is_dir())
        .collect();

    for album in albums {
        process_album(artist, &album)?
    }

    Ok(())
}
