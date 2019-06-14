# fpls

## 说明

复制文件/文件夹

## 使用

```Rust
use fpls_lib::*;
fn main() -> Result<(), io::Error> {
    let path = Path::new("./test/copy_origion");
    let output_path = Path::new("./test/copy_currnet");
    copy_file(&path, &output_path)?;
    Ok(())
}
```

## 运行

```s
cargo run
```
