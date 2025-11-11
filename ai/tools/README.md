# Tool System Architecture

The tool system provides a modular way to add capabilities to the AI agent. Tools are registered based on configuration permissions and can be easily extended.

## Structure

### Core Components

- **`tools.go`**: Defines the `Tool` interface and `Registry` for managing tools
- **`file_tools.go`**: Implements file operation tools (read, write, list directory)

### Tool Interface

Each tool must implement:
```go
type Tool interface {
    Name() string
    Description() string
    Specification() api.Tool
    Execute(ctx context.Context, args json.RawMessage) (string, error)
    RequiresPermission() config.Permission
}
```

### Permission-Based Registration

Tools are automatically filtered based on config permissions:
- `Read`: Enables `read_file` and `list_directory`
- `Write`: Enables `write_file`
- `Command`: Reserved for shell/exec tools (future)

## Available Tools

### read_file
- **Permission**: Read
- **Description**: Read the contents of a file
- **Parameters**: `path` (string)

### list_directory
- **Permission**: Read
- **Description**: List files and directories in a directory
- **Parameters**: `path` (string)

### write_file
- **Permission**: Write
- **Description**: Write or update a file with new content
- **Parameters**: `path`, `content`, `reason` (all strings)

## Adding New Tools

1. Create a new struct implementing the `Tool` interface
2. Register it in `fileops.NewManager()`:
   ```go
   toolRegistry.Register(NewYourTool(cfg))
   ```
3. The registry handles permission checking automatically

## Usage

Tools are available in `/edit` mode (ChatWithTools). The AI can:
- Read files to understand code
- List directories to explore structure
- Write files to make changes (with backup)

Regular chat mode does not have tools enabled to avoid response delays.
