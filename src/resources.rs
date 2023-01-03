use std::io::Read;
use std::path::{Path, PathBuf};
use std::{ffi, fs, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
    FailedToGetAssetDir,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

#[derive(Debug)]
pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        let exe_path = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
        Ok(Resources {
            root_path: exe_path
                .parent()
                .ok_or(Error::FailedToGetAssetDir)?
                .join(rel_path),
        })
    }

    pub fn load_cstring(&self, name: &Path) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(self.root_path.join(name))?;
        let mut buf = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buf)?;
        if buf.iter().find(|b| **b == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buf) })
    }
}
