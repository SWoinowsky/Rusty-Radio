use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use mp3_duration;
use mp3_metadata;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let path = Path::new( "../../Song.mp3" );
    
    let file = BufReader::new(File::open(&path).unwrap());
    let duration = mp3_duration::from_path(&path).unwrap();

    println!("File duration: {:?}", duration);

    let source = Decoder::new(file).unwrap();

    stream_handle.play_raw(source.convert_samples());

    std::thread::sleep(duration);
}
