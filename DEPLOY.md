# ClipType 构建说明

当前项目是 `Tauri 2 + Rust` 桌面应用，构建入口为 `cargo tauri build`。

已确认的项目配置：

- 应用名：`ClipType`
- 包名：`cliptype`
- macOS bundle 目标：可生成 `.app`
- Windows 目标：可生成原生 `cliptype.exe`，也可生成 NSIS 安装包 `.exe`
- 当前 `tauri.conf.json` 里 `bundle.targets = "all"`，在 M 芯片 macOS 上交叉构建 Windows 时需要显式改成 `nsis`，否则会包含 `msi`，而 `msi` 只能在 Windows 上构建

## 构建环境

构建机：`Apple Silicon (M 系列) macOS`

首次准备：

```bash
cargo install tauri-cli --locked
```

如果要在当前这台 M 芯片 Mac 上交叉构建 Windows，还需要：

```bash
brew install llvm nsis
rustup target add x86_64-pc-windows-msvc
cargo install cargo-xwin --locked
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
```

## macOS `.app`

命令：

```bash
cargo tauri build --bundles app
```

产物：

```text
target/release/bundle/macos/ClipType.app
```

说明：

- 在当前 M 芯片 macOS 上，默认产物是 `aarch64-apple-darwin` 版本的 `.app`

## Windows `.exe`

### 方式 1：生成可直接运行的应用程序 `cliptype.exe`

命令：

```bash
cargo xwin build --release --target x86_64-pc-windows-msvc
```

产物：

```text
target/x86_64-pc-windows-msvc/release/cliptype.exe
```

### 方式 2：生成可分发的 Windows 安装包 `.exe`

命令：

```bash
cargo tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --bundles nsis
```

产物：

```text
target/x86_64-pc-windows-msvc/release/bundle/nsis/*.exe
```

说明：

- 如果你的目标是“Windows 可执行文件”，优先使用方式 1
- 如果你的目标是“给用户安装的软件包”，使用方式 2
- 这里显式使用 `--bundles nsis`，是因为当前项目配置里的 `"all"` 在 macOS 交叉构建 Windows 时会把 `msi` 也带上，而 `msi` 不能在 macOS 上生成
