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

    macro_rules! only_ok_directories_in {
        ( $path: expr) => {
            $path
                .filter_map(Result::ok)
                .filter(|d| d.file_type().unwrap().is_dir())
                .map(|d| (d.file_name(), std::fs::read_dir(d.path())))
        };
    }

    for (_, artist) in only_ok_directories_in!(root) {
        for (album_name, album_path) in only_ok_directories_in!(artist.unwrap()) {
            {
                let track_numbers = sorted_track_numbers(album_path.unwrap());

                for n in 1..*track_numbers.last().unwrap_or(&0) {
                    if track_numbers.contains(&n) {
                        continue;
                    }

                    println!(
                        "Album {} missing track number {n:02}",
                        album_name.to_string_lossy()
                    );
                }
            }
        }
    }
}

fn sorted_track_numbers(album: ReadDir) -> Vec<u32> {
    let mut track_numbers: Vec<u32> = Vec::new();

    for file in album.filter_map(Result::ok) {
        // ignore non-musics
        let track_name = match file.path().extension() {
            Some(e) if e == "mp3" || e == "flac" => file.file_name(),
            _ => continue,
        };

        let Ok(track_number) = track_name
            .to_string_lossy()
            .chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
        else {
            continue;
        };

        track_numbers.push(track_number);
    }

    track_numbers.sort_unstable();
    track_numbers
}
