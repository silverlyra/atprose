use std::{fs, io, path::Path};

use globwalk::{GlobWalker, GlobWalkerBuilder};
use serde_json::from_reader;

use super::schema::{self, Schema};

pub fn load(base: impl AsRef<Path>) -> io::Result<Schema> {
    let mut schema = Schema::new();

    for file in glob(base, "*.json") {
        let file = match file {
            Ok(file) => file,
            Err(error) => return Err(io::Error::other(error)),
        };

        let document = load_document(file.path())?;
        schema.insert(document.id.clone(), document);
    }

    Ok(schema)
}

pub fn load_document(path: impl AsRef<Path>) -> io::Result<schema::Document> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let document = from_reader(reader).map_err(io::Error::other)?;
    Ok(document)
}

fn glob(base: impl AsRef<Path>, pattern: impl AsRef<str>) -> GlobWalker {
    GlobWalkerBuilder::new(base, pattern)
        .build()
        .expect("globwalk")
}
