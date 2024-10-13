# basetools

Base tools for developers. These tools are expected to be in the PATH environment variable in some other Dudy projects.

# Installation

## Homebrew

```sh
brew tap dudykr/basetools https://github.com/dudykr/basetools.git
brew install basetools
```

## From Github Actions

As this tool is for providing basic tools for developers, it's fast to install it from Github Actions, too.

```yaml
  - name: Install basetools
    uses: jaxxstorm/action-install-gh-release@v1.10.0
    with: # Grab the latest version
      repo: dudykr/basetools

```

# Usage

## rpx

Run a command from the project root directory. `project_root/bin` will be added to the PATH environment variable before running the command.
If you install binaries to `project_root/bin`, you can use this tool to run them without navigating to the project root directory.

```sh
# Assuming `air` is installed in `project_root/bin` using GOBIN
rpx air
```

will run `air`.
