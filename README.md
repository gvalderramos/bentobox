# BentoBox

![BentoBox Logo](resource/bentobox.png)

A modern and efficient solution for organizing and managing your projects.

## About
BentoBox is a system tray application that helps you organize and quickly access your projects. It creates an icon in your system tray displaying a list of your local projects. With a single click, it launches your preferred IDE (currently tested with VS Code) and opens your selected project, streamlining your development workflow.

Key features:
- System tray integration
- Quick project access
- VS Code integration
- Simple project management

## Configuration

BentoBox creates a configuration file located at `$HOME/.bentobox/bentobox.yaml`. This file allows you to set the root path for your development projects and specify the command for your preferred IDE to open your projects.

### Example Configuration

```yaml
# Configuration for BentoBox
bentobox:
  user-dev-folder: ~/Documents/dev
  ide-cmd: code
```

## Build

To build the project locally:

```bash
# Clone the repository
git clone https://github.com/gvalderramos/bentobox.git

# Navigate to project directory
cd bentobox

# Build the project
cargo build

# Build for release
cargo build --release
```

## Run

To run the project:

```bash
# Development mode
cargo run

# Release mode
cargo run --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.