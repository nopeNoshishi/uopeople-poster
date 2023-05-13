use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Lecture {
    url: String,
    tags: String
}

// impl Lecture {
//     pub fn new(url: String, tags: String) -> Self {
//         Lecture {
//             url,
//             tags
//         }
//     }
// }
