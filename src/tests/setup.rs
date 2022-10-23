use ron::{self, de::from_reader};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub omw_api_key: String,
}

pub fn read_credentials() -> Credentials {
    let file_path: &'static str = "test_data/credentials.ron";
    let file = std::fs::File::open(file_path).expect("Failed opening credentials file.");
    let credentials: Credentials = match from_reader(file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed opening credentials file: {}", e);
            std::process::exit(1);
        }
    };

    credentials
}
