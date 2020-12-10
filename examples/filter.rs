use rfd::{DialogOptions, Filter};

const FILTERS: &[Filter] = &[
    Filter {
        name: "text",
        extensions: &["txt", "rs"],
    },
    Filter {
        name: "rust",
        extensions: &["rs", "toml"],
    },
];

fn main() {
    let params = DialogOptions::new()
        .set_filters(FILTERS)
        .set_starting_directory(&"/");

    let path = rfd::pick_file(params);

    println!(
        "{}",
        path.map_or_else(
            || "The user did not choose any file, or an error occured!".to_owned(),
            |path| format!("The user choose this file: {}", path.to_string_lossy())
        )
    );
}