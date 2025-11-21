use std::{fs::File, io::BufWriter, path::PathBuf};
use url::Url;

pub fn fetch_file(url: &Url, file_path: &PathBuf) -> std::io::Result<u64> {
    let mut binding = ureq::get(url.as_ref()).call().expect("unable to fetch pdf");
    let mut file_data = binding.body_mut().as_reader();

    let writer = File::create(file_path).unwrap();
    let mut writer = BufWriter::new(writer);
    std::io::copy(&mut file_data, &mut writer)
}
