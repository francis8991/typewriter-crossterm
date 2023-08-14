pub const TOP_250: &'static str = include_str!("words/top250");

pub fn get_word_list(name: &str) -> Option<&'static str> {
    match name {
        "top250" => Some(TOP_250),
        _ => None,
    }
}
