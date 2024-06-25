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

    for file in root.filter_map(Result::ok) {
        // ignore non-directories
        let artist = match file.file_type() {
            Ok(d) if d.is_dir() => std::fs::read_dir(file.path()).unwrap(),
            _ => continue,
        };

        for file in artist.filter_map(Result::ok) {
            // ignore non-directories
            let album = match file.file_type() {
                Ok(d) if d.is_dir() => std::fs::read_dir(file.path()).unwrap(),
                _ => continue,
            };

            println!("Album: {album:?}")
        }
    }
}
