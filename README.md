# Notia (Open Source GNOME Edition)

![Screenshot](https://github.com/user-attachments/assets/269c4b88-a5e3-4de7-8ec8-e4f20ee23dc5)

**Notia** is a free and open-source application for Linux, designed to combine a **photo gallery** with a **note-taking system**.  
It's built with **Rust, GTK4 and Libadwaita** to provide a fast, lightweight and GNOME-native experience.

## ✨ Features

### ✅ Currently Available
- 📸 **Photo Gallery**: Browse and organize your photo collection
- 📝 **Note Taking**: Write and save notes linked to each photo
- 🔖 **Tag System**: Add and manage tags for photos
- 🎨 **GNOME Native UI**: Adaptive interface following GNOME Human Interface Guidelines
- 💾 **Local Storage**: Notes and tags are saved locally in JSON format
- 🔄 **Auto-scan**: Automatically scans Pictures, Downloads, and other common directories

### 🚧 Planned Features
- 🔍 **Search**: Search photos by tags and notes
- ☁️ **Sync**: Optional cloud sync and backup support
- 📱 **Mobile**: Companion mobile app
- 🎯 **Advanced Organization**: Albums and collections

## 🚀 Tech Stack
- **[Rust](https://www.rust-lang.org/)** - Fast, safe, and concurrent programming language
- **[GTK4](https://www.gtk.org/)** - Modern toolkit for creating graphical user interfaces
- **[Libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/)** - GNOME's design system library
- **[Meson](https://mesonbuild.com/)** - Build system
- **JSON** - Local data storage (notes and tags)

## 📦 Installation

### Prerequisites
- Linux distribution with GNOME or GTK4 support
- Rust toolchain (rustc, cargo)
- Meson build system
- GTK4 development libraries
- Libadwaita development libraries

### Building from Source

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/notia.git
   cd notia
   ```

2. **Install dependencies**
   
   **Arch Linux / Manjaro:**
   ```bash
   sudo pacman -S rust meson gtk4 libadwaita
   ```
   
   **Ubuntu / Debian:**
   ```bash
   sudo apt install rustc cargo meson libgtk-4-dev libadwaita-1-dev
   ```
   
   **Fedora:**
   ```bash
   sudo dnf install rust meson gtk4-devel libadwaita-devel
   ```

3. **Build the project**
   ```bash
   meson setup build
   ninja -C build
   ```

4. **Run the application**
   ```bash
   ./build/src/notia
   ```

### Installation (System-wide)
```bash
sudo ninja -C build install
```

## 🎯 Usage

1. **Launch Notia** - The application will automatically scan your photo directories
2. **Browse Photos** - Use the gallery grid to browse your photos
3. **Select a Photo** - Click on any photo to view it in detail
4. **Add Notes** - Use the sidebar to write notes about the selected photo
5. **Add Tags** - Use the tag entry field to add tags to your photos
6. **Save** - Click the save button to persist your notes and tags
7. **Navigate** - Use the previous/next buttons or sidebar toggle as needed

## 🗂️ Data Storage

Notia stores all your notes and tags locally in:
- **Notes File**: `~/.notia_notes.json`
- **Scanned Directories**: Pictures, Downloads, and other common photo directories

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Commit your changes** (`git commit -m 'Add amazing feature'`)
4. **Push to the branch** (`git push origin feature/amazing-feature`)
5. **Open a Pull Request**

### Development Setup
```bash
# Clone and setup
git clone https://github.com/your-username/notia.git
cd notia

# Install development dependencies
sudo pacman -S rust meson gtk4 libadwaita

# Build and run
meson setup build
ninja -C build
./build/src/notia
```

## 🐛 Reporting Issues

Found a bug? Please report it on our [Issues page](../../issues) with:
- Your operating system and version
- Steps to reproduce the issue
- Expected vs actual behavior
- Any error messages

## 📜 License

Notia is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html) (GPL-3.0).  
You are free to use, modify, and distribute this software under the same license.

## 🙏 Acknowledgments

- GNOME Foundation for GTK4 and Libadwaita
- Rust community for the excellent ecosystem
- All contributors and users of Notia

---

**Made with ❤️ for the Linux community**

