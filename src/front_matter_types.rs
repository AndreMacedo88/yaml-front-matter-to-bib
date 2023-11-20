pub mod article_bio_like;

pub trait GenerateBibLines {
    fn generate_bib_lines(&self);
}

fn article_bio_like_() {
    pub use article_bio_like::{generate_bib_lines, Metadata};
}

pub fn get_type_objects(arg: String) {
    match arg.as_str() {
        "article_bio_like" => article_bio_like_(),
        _ => panic!("Type not yet implemented"),
    }
}

// fn article_bio_like_() -> (fn(), fn()) {
//     pub use article_bio_like::{generate_bib_lines, Metadata};
//     (
//         || Metadata {
//             title: String,
//             author: String,
//             journal: String,
//             year: u16,
//             volume: u32,
//             number: u32,
//             pages: String,
//         },
//         generate_bib_lines,
//     )
// }

// pub fn get_type_objects(arg: String) -> (fn(), fn()) {
//     match arg.as_str() {
//         "article_bio_like" => article_bio_like_(),
//         _ => panic!("Type not yet implemented"),
//     }
// }
