use std::fs::ReadDir;

use clap::Arg;

fn main() {
    let matches = clap::command!()
        .arg(
            Arg::new("path")
                .required(true)
                .value_name("PATH")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Path to music library root"),
        )
        .get_matches();

    let root = {
        let path = matches.get_one::<String>("path").unwrap().to_string();
        std::fs::read_dir(path).expect("Failed to open specified directory")
    };

    for file in root
        .filter_map(Result::ok)
        .filter(|d| d.file_type().unwrap().is_dir())
    {
        // ignore non-directories
        let artist = std::fs::read_dir(file.path()).unwrap();

        for album in artist
            .filter_map(Result::ok)
            .filter(|d| d.file_type().unwrap().is_dir())
            .filter_map(|d| Some(std::fs::read_dir(d.path())))
            .filter_map(Result::ok)
        {
            let track_numbers = compile_track_numbers(album);
        }
    }
}

fn compile_track_numbers(album: ReadDir) -> Vec<u32> {
    let mut track_numbers: Vec<u32> = Vec::new();

    for file in album.filter_map(Result::ok) {
        // ignore non-musics
        let track_name = match file.path().extension() {
            Some(e) if e == "mp3" || e == "flac" => file.file_name(),
            _ => continue,
        };

        let track_number = track_name.to_string_lossy()[..2].parse::<u32>().unwrap();
        track_numbers.push(track_number);
    }

    track_numbers
}
