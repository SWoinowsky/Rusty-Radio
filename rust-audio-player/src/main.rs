use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use mp3_duration;
use mp3_metadata;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // let path = Path::new( "../../Song.mp3" );

    let paths = std::fs::read_dir("../..").unwrap();



    for dir_path in paths {

    let path = dir_path.unwrap().path();

    let file = Path::new( &path );
    if file.is_file() {
    
    let file = BufReader::new(File::open(&path).unwrap());
    let duration = mp3_duration::from_path(&path).unwrap();

    let metadata = mp3_metadata::read_from_file(&path).unwrap();


    println!("File duration: {:?}", metadata.duration);
    
    if let Some(tag) = metadata.tag {
        println!("title: {}", tag.title);
        println!("artist: {}", tag.artist);
        println!("album: {}", tag.album);
        println!("year: {}", tag.year);
        println!("comment: {}", tag.comment);
        println!("genre: {:?}", tag.genre);
    } else {
        println!("No tag");
    }

    let source = Decoder::new(file).unwrap();

    stream_handle.play_raw(source.convert_samples());

    std::thread::sleep(metadata.duration);

    }
    }
}
