pub mod client;
pub use client::{Dynasty, Error, Result};

pub mod http;
pub use http::Http;

pub mod model;
pub use model::{Author, BrowseTag as Tag, Chapter, Chapters, Pairing, Series};

pub mod path;
pub use path::Path;
