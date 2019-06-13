use std::io;
use std::fs;
use std::path::Path;

/// 复制文件
/// 
/// # Examples
/// 
/// ``` no_run
/// 
/// fn main() -> Result<(), io::Error> {
///     let path = Path::new("./test/copy_origion");
///     let output_path = Path::new("./test/copy_currnet");
///     copy_file(&path, &output_path)?;
///     Ok(())
/// }
/// ```
pub fn copy_file(path: &Path, output_path: &Path) -> Result<(), io::Error> {
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let oup = output_path.join(entry.file_name());
            fs::create_dir(&oup)?;
            copy_file(path.as_path(), &oup.as_path())?;
        } else {
            fs::copy(path, output_path.join(entry.file_name()))?;
        }
    }
    Ok(())
}
