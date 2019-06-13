# fpls

## 说明

复制文件/文件夹

## 配置文件

fpls.config.json

- path 要复制的路径
- output_path 复制到什么地方

```json
{
  "path": "./test/copy_origion",
  "output_path": "./test/copy_currnet"
}
```

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
