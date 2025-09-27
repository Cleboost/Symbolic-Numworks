<div align="center">
<h1>🧮 Numworks Application Template (RUST)</h1>

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Numworks](https://img.shields.io/badge/Numworks-Calculator-blue.svg?style=for-the-badge)](https://www.numworks.com/)
[![License](https://img.shields.io/badge/license-MIT-green.svg?style=for-the-badge)](LICENSE)


> A simple yet powerful Rust template for developing applications on Numworks calculators
</div>

## 📋 Overview

This template provides a minimal foundation for creating Rust applications that run natively on Numworks calculators (N110, N115, N120). It includes a "Hello World" example that displays perfectly centered text on the screen.

## ✨ Features

- 🦀 **Rust `no_std`** configuration for embedded calculators
- 🎯 **ARM cross-compilation** with `thumbv7em-none-eabihf`
- 🔧 **EADK Numworks SDK** integration
- 🖥️ **Epsilon simulator** support
- ⚡ **Build system** with Just
- 🎨 **Working Hello World** example with perfect text centering
- 📱 **Multi-calculator support** (N110, N115, N120)

## 📦 Prerequisites

To build this application, you need to install:

- 🦀 **Rust compiler** with embedded ARM support
- 🔨 **[Arm GCC compiler](https://developer.arm.com/downloads/-/gnu-rm)**
- 📦 **[Node.js](https://nodejs.org/en/)**
- 🛠️ **EADK Numworks SDK** via [nwlink](https://www.npmjs.com/package/nwlink)

> ⚠️ **Important:** Make sure `arm-none-eabi-gcc` is in your PATH.

For detailed SDK installation instructions, follow [this guide](https://www.numworks.com/engineering/software/build/).

## 🚀 Installation

```shell
# On macOS with Homebrew
brew install rustup node

# Or equivalent on your OS
rustup-init
rustup target add thumbv7em-none-eabihf
cargo install just # Makefile alternative
```

## 🎮 Usage

### 🔨 Build the application
```shell
just build
```

### 📱 Send to calculator
Connect your calculator and run:
```shell
just send
```

### 🖥️ Run on simulator
```shell
just sim
```

You can speed up simulator compilation by specifying the number of jobs:
```shell
just sim 5
```

## ⚠️ Known Issues

### 🖥️ Simulator Video Error
If you encounter a video error when launching the simulator, use this command to force X11 driver usage:

```shell
export SDL_VIDEODRIVER=x11
just sim
```

This command resolves video compatibility issues on some Linux systems.

## 📁 Project Structure

```
📦 num-template/
├── 📁 src/
│   ├── 📄 main.rs          # 🚀 Main entry point
│   ├── 📄 eadk.rs          # 🔧 EADK SDK bindings
│   └── 📁 libs/            # 📚 C libraries (storage)
│       ├── 📄 storage.c
│       └── 📄 storage.h
├── 📁 assets/
│   └── 🖼️ icon.png         # 🎨 Application icon (55x55 PNG)
├── 📄 build.rs             # ⚙️ Custom build script
├── 📄 Cargo.toml           # 🦀 Rust configuration
└── 📄 justfile             # 🔨 Build commands
```

## 🛠️ Development

### 🎨 Customizing the application
Edit `src/main.rs` to customize your application. The current example displays "Hello World!" with perfectly centered text and waits for the EXE key to exit.

#### 🖼️ Important: Icon Size Configuration
When you replace the default icon with your own, you **must** update the array size in `main.rs`:

```rust
// Change this size to match your icon's byte size
pub static EADK_APP_ICON: [u8; 1520] = *include_bytes!("../target/icon.nwi");
```

**How to find your icon size:**
```shell
# Check the size of your generated icon
ls -la target/icon.nwi
```

If the compilation fails with a "mismatched types" error, it means the array size doesn't match your icon's actual size. Update the number in the brackets `[u8; YOUR_SIZE_HERE]` to fix this.

### ➕ Adding features
- 🔧 Use EADK APIs in `src/eadk.rs` for display, input, etc.
- 📦 Add your dependencies in `Cargo.toml`
- ⚡ Respect `no_std` constraints for calculator compatibility

### 📱 Calculator support
- **N110/N115**: ✅ Full support
- **N120**: ✅ Full support with better performance
- **Simulator**: 🖥️ Development and testing

## 🔍 Key Features Explained

### 🎯 Text Centering
The template includes a `estimate_text_width()` function that calculates text width based on character count and font size, enabling perfect horizontal centering on the 320x240 calculator screen.

### 🌐 Cross-platform Development
- **Calculator**: 🎯 ARM Cortex-M target with `no_std`
- **Simulator**: 🖥️ Native platform for development and testing
- **Build system**: ⚡ Automated cross-compilation and deployment

### 💾 Memory Management
- 🔄 Embedded allocator for heap management
- ⚡ Optimized for calculator memory constraints
- 🛡️ Safe memory operations with overflow protection

## 🚀 Getting Started

1. 📥 Clone this template
2. 📦 Install prerequisites
3. 🖼️ Add your `assets/icon.png` (16x16 PNG)
4. ✏️ Modify `src/main.rs` for your application
5. 🔨 Build with `just build`
6. 🧪 Test with `just sim` or deploy with `just send`

## 🤝 Contributing

This template is designed to be a starting point for Numworks calculator applications. Feel free to:
- ➕ Add more examples
- ⚡ Improve the build system
- 🛠️ Add utility functions
- 📱 Create application templates for specific use cases

## 📄 License

This template is provided as-is for educational and development purposes. Please respect Numworks' terms of service when developing applications for their calculators.

## 📚 Source

This template was inspired by the amazing work done in [NumcraftRust](https://github.com/yannis300307/NumcraftRust) - a Minecraft-like game running natively on Numworks calculators. The project demonstrates the incredible potential of Rust for embedded calculator development and serves as an excellent reference for advanced Numworks applications.
