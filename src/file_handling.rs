use std::{fs::File, io::BufWriter, path::PathBuf};
use url::Url;

pub fn fetch_file(url: &Url, file_path: &PathBuf) -> std::io::Result<u64> {
    let mut file_data = ureq::get(&url.to_string())
        .call()
        .expect("unable to fetch pdf")
        .into_reader();

    let writer = File::create(file_path).unwrap();
    let mut writer = BufWriter::new(writer);
    std::io::copy(&mut file_data, &mut writer)
}
