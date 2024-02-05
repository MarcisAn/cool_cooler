use hdf5::{Dataset, File};

pub fn open_file(path: &str) -> File {
    let file = File::open(path);
    if file.is_err() {
        panic!("Cannot read file \"{}\"", path);
    }
    return file.unwrap();
}
pub fn get_dataset(file: &File, set_name: &str) -> Dataset {
    let ds = file.dataset(&set_name);
    if ds.is_err() {
        panic!("Cannot open dataset \"{}\"", set_name);
    }
    return ds.unwrap();
}
