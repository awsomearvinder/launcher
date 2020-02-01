#![allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binary {
    name: std::ffi::OsString,
    path: std::path::PathBuf,
}

impl Binary {
    /// Runs the binary as a child process - returns child process upon success.
    pub fn run(&self) -> std::io::Result<std::process::Child> {
        Ok(std::process::Command::new(self.path.clone().into_os_string()).spawn()?)
    }
    /// Makes a new Binary type - use if specifying custom path.
    pub fn new(name: std::ffi::OsString, path: std::path::PathBuf) -> Self {
        Binary {
            name: name,
            path: path,
        }
    }
    /// Returns the name as OsString
    pub fn name(&self) -> &std::ffi::OsString {
        &self.name
    }
    /// Returns Path as PathBuf.
    pub fn path(&self) -> &std::path::PathBuf {
        &self.path
    }
    ///Gets a list of binaries
    pub fn get_binaries() -> Vec<Binary> {
        let bin_directories = ["/usr/bin", "/usr/local/bin"];
        let mut bins = Vec::new();
        for dir in bin_directories.iter() {
            let dir_iter = std::fs::read_dir(dir).unwrap_or_else(|e| {
                eprintln!("error: {}", e);
                std::process::exit(1);
            });
            for bin in dir_iter {
                let bin = bin.unwrap_or_else(|e| {
                    eprintln!("error : {}", e);
                    std::process::exit(1);
                });
                bins.push(Binary::new(bin.file_name(), bin.path()));
            }
        }
        bins.dedup();
        bins
    }
    ///Gets binaries without duplicated names.
    pub fn get_binaries_dedup() -> Vec<Binary> {
        let mut bins = Binary::get_binaries();
        bins.dedup();
        bins
    }
}
