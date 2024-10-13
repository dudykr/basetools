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

You can use this to avoid installing binaries globally. e.g.

```sh
export GOBIN="$PWD/bin"

# Buf & protoc plugins
go install github.com/bufbuild/buf/cmd/buf@latest
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install connectrpc.com/connect/cmd/protoc-gen-connect-go@latest

# Other tools
go install github.com/sqlc-dev/sqlc/cmd/sqlc@latest
go install github.com/air-verse/air@latest
go install github.com/jackc/tern/v2@latest

```

This script will install the tools in the current directory's `bin` folder. But normally you can't run it from terminal without specifying the full path. `rpx` solves this problem. `rpx air` will run `air` from the project `bin` directory.