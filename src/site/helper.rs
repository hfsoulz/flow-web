// luflow.net web site
// AGPL-3.0 License (see LICENSE)

use regex::Regex;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

pub struct Helper {}

impl Helper {
    pub fn get_current_working_dir() -> PathBuf {
        let res = env::current_dir();
        match res {
            Ok(path) => path,
            Err(_) => PathBuf::new(),
        }
    }

    pub fn get_output_dir() -> PathBuf {
        let cwd = Helper::get_current_working_dir();
        return cwd.join("output");
    }

    /// Replaces spaces with '-' and only allows 'a-z', 'A-Z', '0-9' and '-' characters and
    /// converts to lowercase.
    ///
    /// # Arguments
    ///
    /// * `str` - is the string to sanitize.
    ///
    /// # Examples
    ///
    /// ```
    /// let str_to_sanitize = "This is a test, and so on.";
    /// let str_sanitized = Helper::sanitize_string(str_to_sanitize);
    /// assert_eq!(str_sanitized, "this-is-a-test-and-so-on");
    /// ```
    pub fn sanitize_string(str: &str) -> String {
        let re_whitespace = Regex::new(r"\s").unwrap();
        let re_unsupported = Regex::new(r"[^0-9a-zA-Z-]").unwrap();

        // replace all spaces with '-':
        let after_whitespace = re_whitespace.replace_all(str, "-");
        // only allow 'a-z' 'A-Z' '0-9' and '-' characters:
        let after_unsupported = re_unsupported.replace_all(&after_whitespace, "");

        // also convert to lower case:
        return after_unsupported.to_string().to_lowercase();
    }

    pub fn create_dir_all<'a>(dir: &'a PathBuf) {
        match fs::create_dir_all(dir) {
            Ok(()) => println!("Created dir: '{}'", dir.display()),
            Err(err) => panic!(
                "Failed to create dir: '{}'. Error msg: '{}'",
                dir.display(),
                err
            ),
        };
    }

    pub fn remove_dir_all<'a>(dir: &'a PathBuf) {
        match fs::remove_dir_all(dir) {
            Ok(()) => println!("Removed dir: '{}'", dir.display()),
            Err(err) => panic!(
                "Failed to remove dir: '{}'. Error msg: '{}'",
                dir.display(),
                err
            ),
        };
    }

    pub fn exists_dir<'a>(dir: &'a PathBuf) -> bool {
        return dir.as_path().exists();
    }

    #[async_recursion::async_recursion]
    pub async fn copy_dir_all<S, D>(src: S, dst: D) -> Result<(), std::io::Error>
    where
        S: AsRef<Path> + Send + Sync,
        D: AsRef<Path> + Send + Sync,
    {
        tokio::fs::create_dir_all(&dst).await?;
        let mut entries = tokio::fs::read_dir(src).await?;
        while let Some(entry) = entries.next_entry().await? {
            let ty = entry.file_type().await?;
            if ty.is_dir() {
                Self::copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name())).await?;
            } else {
                tokio::fs::copy(entry.path(), dst.as_ref().join(entry.file_name())).await?;
            }
        }
        Ok(())
    }

    pub async fn write_file<'a>(file_path: &'a PathBuf, data: &'a [u8]) -> io::Result<()> {
        // create output file:
        let mut file = File::create(file_path).await?;

        // write data to file:
        file.write_all(data).await?;

        println!("Wrote '{}' successfully", file_path.display());
        Ok(())
    }

    pub fn write_file_sync<'a>(file_path: &'a PathBuf, data: &'a [u8]) -> io::Result<()> {
        // create output file:
        let mut file = std::fs::File::create(file_path)?;

        // write data to file:
        file.write_all(data)?;

        println!("Wrote '{}' successfully", file_path.display());
        Ok(())
    }
}
