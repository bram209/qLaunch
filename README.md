# qLaunch
Quick app launcher for linux

![Image of qLaunch](https://github.com/bram209/qLaunch/blob/master/qlauncher-screenshot.png?raw=true)

## Installation

### Required package
#### Solus
```
sudo eopkg install libgtk-3-devel
```
#### Debian & Ubuntu
```
sudo apt-get install libgtk-3-dev
```
#### Fedora
```
sudo dnf install gtk3-devel glib2-devel
```

### Get up and running
#### 1. Install Rust
```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

#### 2. Clone repository
```
git clone https://github.com/bram209/qLaunch.git
```

#### 3. Build
```
cd qLaunch
cargo build --release
```

#### 4. Copy to bin folder (optional)
```
sudo cp target/release/app-launcher /usr/bin/
```

#### 5. Make a new shortcut to run the app
I use alt + F2


## Disclaimer
I am new to Rust and system programming in general.
The state of the code right now is a bit messy but will be cleaned up later.
Any feedback or suggestions are welcome.

