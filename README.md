# basetools

Base tools for developers. These tools are expected to be in the PATH environment variable in some other Dudy projects.

# Installation

Grab the latest release from the [Releases](https://github.com/dudykr/basetools/releases) page. We will provide an installation script soon.

# Usage

## rpx

Run a command from the project root directory. `project_root/bin` will be added to the PATH environment variable before running the command.
If you install binaries to `project_root/bin`, you can use this tool to run them without navigating to the project root directory.

```sh
# Assuming `air` is installed in `project_root/bin` using GOBIN
rpx air
```

will run `air`.
