mod hdf_loader;
use hdf5::{Dataset, File};
use hdf_loader::{get_dataset, open_file};

pub struct CoolCooler {
    file: File,
}

impl CoolCooler {
    pub fn new(file_path: &str) -> CoolCooler {
        let file = open_file(file_path);
        return CoolCooler { file };
    }
    pub fn get_datasets(self) -> Vec<Dataset> {
        self.file.datasets().unwrap()
        //return get_dataset(&self.file, dataset_name);
    }
}

#[cfg(test)]
mod tests {
    use super::CoolCooler;

    #[test]
    fn it_works() {
        let cooler = CoolCooler::new("test2.cool");
        cooler.get_datasets();
        //read_hdf5();
    }
}
