# ops-lens: System & Cloud Observability

A high-performance terminal UI application for tailing and monitoring log files with customizable keybinds and intelligent log level detection.

## Features

✨ **Real-time Log Tailing** - Stream log files with live updates
🎨 **Intelligent Log Level Detection** - Automatic color-coding based on log patterns
⌨️ **Customizable Keybinds** - Configure keyboard shortcuts to match your preferences
🔧 **Zero Configuration** - Works out of the box with sensible defaults
📦 **Multi-Provider Support** - Works with any logging provider (AWS, GCP, Azure, Datadog, custom apps)
🚀 **High Performance** - Efficient pattern matching and rendering

## Quick Start

### Installation

```bash
cargo build --release
```

### Running

```bash
cargo run
```

On first run, `ops-lens.toml` will be automatically created with default configuration.

### Basic Usage

1. **Press 'e'** to enter edit mode and specify a log file path
2. **Press 'j'** to scroll down
3. **Press 'k'** to scroll up
4. **Press 'q'** to quit

All keybinds are customizable via `ops-lens.toml`

## Configuration

### Configuration File Location

The `ops-lens.toml` file should be placed in the directory where you run the application.

```
your-project/
├── ops-lens (executable or cargo run)
└── ops-lens.toml ← configuration file
```

If the file doesn't exist, ops-lens will automatically create one with default settings on first run.

## Configuration Guide

### Keybinds Configuration

The `[keybinds]` section defines all keyboard shortcuts for controlling the application.

#### Available Commands

- `quit` - Exit the application
- `edit` - Enter edit mode to change the log file path
- `scroll_up` - Scroll up in the log viewer
- `scroll_down` - Scroll down in the log viewer

#### Default Keybinds (Vim-style)

```toml
[keybinds]
quit = "q"              # Quit the application
edit = "e"              # Edit the log file path
scroll_up = "k"         # Scroll up in the log view
scroll_down = "j"       # Scroll down in the log view
```

#### Customizing Keybinds

You can customize any keybind to any single character (a-z, A-Z, 0-9):

```toml
[keybinds]
quit = "q"
edit = "e"
scroll_up = "w"         # Use 'w' instead of 'k'
scroll_down = "s"       # Use 's' instead of 'j'
```

#### Common Keybind Examples

**WASD Navigation:**
```toml
[keybinds]
quit = "q"
edit = "e"
scroll_up = "w"
scroll_down = "s"
```

**Arrow Key Alternative:**
```toml
[keybinds]
quit = "q"
edit = "e"
scroll_up = "u"
scroll_down = "d"
```

**Custom Layout:**
```toml
[keybinds]
quit = "x"
edit = "e"
scroll_up = "p"
scroll_down = "n"
```

### Log Level Configuration

The `[log_level]` section controls how log level information is displayed and detected.

#### Display Options

```toml
[log_level]
enabled = true                      # Enable/disable log level indicator
indicator_position = "top"          # Position: "top" or "bottom"
show_level_counts = true            # Show count of each log level
```

- `enabled`: Boolean - Enable or disable the log level indicator display
- `indicator_position`: String ("top" or "bottom") - Where to show the indicator in the UI
- `show_level_counts`: Boolean - Display the count of each log level type

### Log Level Patterns

Configure which keywords or phrases indicate each log level. Each level can have multiple patterns that are matched case-insensitively.

#### Error Level Configuration

```toml
[log_level.levels.error]
patterns = ["ERROR", "SEV", "CRITICAL", "FATAL"]
color = "red"
display_name = "ERROR "
```

- `patterns` - Array of keywords to match (case-insensitive)
- `color` - Display color for matched lines
- `display_name` - Text shown in the log level indicator

#### Warning Level Configuration

```toml
[log_level.levels.warning]
patterns = ["WARN", "WARNING"]
color = "yellow"
display_name = "WARN  "
```

#### Info Level Configuration

```toml
[log_level.levels.info]
patterns = ["INFO"]
color = "green"
display_name = "INFO  "
```

#### Debug Level Configuration

```toml
[log_level.levels.debug]
patterns = ["DEBUG", "TRACE"]
color = "blue"
display_name = "DEBUG "
```

#### Supported Colors

- `red`, `yellow`, `green`, `blue`, `cyan`, `magenta`, `white`
- `gray` / `grey`
- `dark_gray` / `dark_grey`

### Complete Default Configuration

```toml
# ops-lens Configuration File
# This file contains settings for keybinds and log level patterns

[keybinds]
# Navigation and control keybinds
quit = "q"              # Quit the application
edit = "e"              # Edit the log file path
scroll_up = "k"         # Scroll up in the log view
scroll_down = "j"       # Scroll down in the log view

[log_level]
# Log level indicator settings
enabled = true                      # Enable/disable log level indicator
indicator_position = "top"          # Position: "top" or "bottom"
show_level_counts = true            # Show count of each log level

# Error level patterns - customize for your log provider
[log_level.levels.error]
patterns = ["ERROR", "SEV", "CRITICAL"]  # Patterns to match ERROR level
color = "red"                            # Display color
display_name = "ERROR "                  # Text to display in log indicator

# Warning level patterns - customize for your log provider
[log_level.levels.warning]
patterns = ["WARN", "WARNING"]      # Patterns to match WARNING level
color = "yellow"                    # Display color
display_name = "WARN  "             # Text to display in log indicator

# Info level patterns - customize for your log provider
[log_level.levels.info]
patterns = ["INFO"]                 # Patterns to match INFO level
color = "green"                     # Display color
display_name = "INFO  "             # Text to display in log indicator

# Debug level patterns - customize for your log provider
[log_level.levels.debug]
patterns = ["DEBUG", "TRACE"]       # Patterns to match DEBUG level
color = "blue"                      # Display color
display_name = "DEBUG "             # Text to display in log indicator
```

## Configuration Examples

### Example 1: AWS CloudWatch Logs

```toml
[keybinds]
quit = "q"
edit = "e"
scroll_up = "k"
scroll_down = "j"

[log_level]
enabled = true
indicator_position = "top"
show_level_counts = true

[log_level.levels.error]
patterns = ["ERROR", "FATAL"]
color = "red"
display_name = "ERROR "

[log_level.levels.warning]
patterns = ["WARN", "WARNING"]
color = "yellow"
display_name = "WARN  "

[log_level.levels.info]
patterns = ["INFO"]
color = "green"
display_name = "INFO  "

[log_level.levels.debug]
patterns = ["DEBUG"]
color = "blue"
display_name = "DEBUG "
```

### Example 2: Google Cloud Logging

```toml
[log_level.levels.error]
patterns = ["ERROR", "CRITICAL", "ALERT", "EMERGENCY"]
color = "red"
display_name = "ERROR "

[log_level.levels.warning]
patterns = ["WARNING", "NOTICE"]
color = "yellow"
display_name = "WARN  "

[log_level.levels.info]
patterns = ["INFO", "DEFAULT"]
color = "green"
display_name = "INFO  "

[log_level.levels.debug]
patterns = ["DEBUG"]
color = "blue"
display_name = "DEBUG "
```

### Example 3: Multi-Provider Support

Combine patterns from multiple providers:

```toml
[log_level.levels.error]
patterns = ["ERROR", "SEV", "CRITICAL", "FATAL", "PANIC", "ERR", "SEVERE"]
color = "red"
display_name = "ERROR "

[log_level.levels.warning]
patterns = ["WARN", "WARNING", "NOTICE", "WRN"]
color = "yellow"
display_name = "WARN  "

[log_level.levels.info]
patterns = ["INFO", "INFORMATION", "INF"]
color = "green"
display_name = "INFO  "

[log_level.levels.debug]
patterns = ["DEBUG", "TRACE", "DBG", "VERBOSE"]
color = "blue"
display_name = "DEBUG "
```

### Example 4: Custom Application Patterns

For applications with custom log patterns:

```toml
[log_level.levels.error]
patterns = ["FAILURE", "FAILED", "FATAL", "CRASH", "EXCEPTION"]
color = "red"
display_name = "ERROR "

[log_level.levels.warning]
patterns = ["ALERT", "CAUTION", "RETRY"]
color = "yellow"
display_name = "WARN  "

[log_level.levels.info]
patterns = ["SUCCESS", "COMPLETED", "STARTED"]
color = "green"
display_name = "INFO  "

[log_level.levels.debug]
patterns = ["DETAIL", "DIAGNOSTIC"]
color = "blue"
display_name = "DEBUG "
```

### Example 5: Minimal Configuration

Disable log level display for a cleaner interface:

```toml
[keybinds]
quit = "q"
edit = "e"
scroll_up = "k"
scroll_down = "j"

[log_level]
enabled = false
```

### Example 6: Custom Keybinds with WASD

```toml
[keybinds]
quit = "q"
edit = "e"
scroll_up = "w"
scroll_down = "s"

[log_level]
enabled = true
indicator_position = "bottom"
show_level_counts = true
```

## Pattern Matching Details

### How Pattern Matching Works

1. Each log line is checked against error patterns first (highest priority)
2. Then warning patterns, info patterns, and debug patterns
3. First matching pattern determines the log level and color
4. Unmatched lines use default styling

### Case-Insensitive Matching

Patterns are matched case-insensitively, so a pattern "error" will match:
- "ERROR"
- "Error"
- "error"
- "ErRoR"

### Multiple Patterns Per Level

Each level supports multiple patterns:

```toml
[log_level.levels.error]
patterns = ["ERROR", "SEV", "CRITICAL", "FATAL", "PANIC"]
```

A log line matching any of these patterns will be displayed as an error.

## User Interface Layout

### Layout with Log Level Indicator at Top (Default)

```
┌─────────────────────────────────────────┐
│ OpsLens: System & Cloud Observability   │
├─────────────────────────────────────────┤
│ Tail File (Press 'e' to edit): path     │
├─────────────────────────────────────────┤
│ Levels: 5 ERROR | 12 WARNING | 23 INFO  │
├─────────────────────────────────────────┤
│                                         │
│ ERROR Database connection failed        │
│ WARN  Retry attempt 1 of 3             │
│ INFO  Connection established            │
│ DEBUG Query execution time: 245ms       │
│                                         │
├─────────────────────────────────────────┤
│ ❌ File not found: 'path/to/file'       │
├─────────────────────────────────────────┤
│ [q] quit | [e] edit | [k] up | [j] dn  │
└─────────────────────────────────────────┘
```

### Layout with Log Level Indicator at Bottom

```
┌─────────────────────────────────────────┐
│ OpsLens: System & Cloud Observability   │
├─────────────────────────────────────────┤
│ Tail File (Press 'e' to edit): path     │
├─────────────────────────────────────────┤
│                                         │
│ ERROR Database connection failed        │
│ WARN  Retry attempt 1 of 3             │
│ INFO  Connection established            │
│ DEBUG Query execution time: 245ms       │
│                                         │
├─────────────────────────────────────────┤
│ Levels: 5 ERROR | 12 WARNING | 23 INFO  │
├─────────────────────────────────────────┤
│ ❌ File not found: 'path/to/file'       │
├─────────────────────────────────────────┤
│ [q] quit | [e] edit | [k] up | [j] dn  │
└─────────────────────────────────────────┘
```

### UI Components

- **Title Bar**: Shows application name (always visible)
- **Input Box**: Current log file path with edit mode indicator
- **Log Level Indicator**: Shows counts of each log level (configurable position: top or bottom)
- **Logs Area**: Color-coded log lines with level-based colors
- **Error/Status Line**: Error messages displayed in red with ❌ indicator (second line from bottom)
- **Keybinds Help**: Current keybinds on the last line (always visible)

## Troubleshooting

### Configuration File Not Being Read

1. **Check file location**: Ensure `ops-lens.toml` is in the same directory where you run the application
2. **Check file permissions**: Make sure the file is readable
3. **Check TOML syntax**: Ensure the TOML file is properly formatted
4. **Check for error messages**: ops-lens will print warnings if it fails to load the config file

### Invalid Configuration Errors

If the configuration file has syntax errors:
- ops-lens will print a warning message indicating the parse error
- The application will fall back to default settings
- Review the error message to identify the issue

### Changes Not Taking Effect

- Changes to the configuration file require a restart of the application
- Make sure to save the file after editing
- Configuration is only loaded at application startup

### Logs Not Being Colored Correctly

1. Check the exact keyword in your logs
2. Verify the pattern in your configuration file matches the keyword
3. Consider case variations - try adding multiple patterns:
   ```toml
   patterns = ["ERROR", "Error", "error"]
   ```
4. Remember patterns are checked in priority order (error → warning → info → debug)

### Some Logs Getting Wrong Color

This usually means patterns are overlapping. The first matching pattern wins:

```toml
# This might cause issues if "CRITICAL" appears in logs
[log_level.levels.error]
patterns = ["CRITICAL", "ERROR"]  # CRITICAL checked first - good

[log_level.levels.warning]
patterns = ["WARN"]               # OK - no overlap
```

## TOML Syntax Reference

### Basic Syntax

```toml
# Comments start with #

[section_name]
# Key-value pairs use = assignment
key = "value"
boolean_key = true
number_key = 42

# Arrays
array_key = ["item1", "item2", "item3"]

# Strings must be quoted
string_key = "this is a string"
```

### String Values

All configuration values must be properly quoted strings:

```toml
# Correct:
quit = "q"
color = "red"

# Incorrect:
quit = q          # Missing quotes
color = red       # Missing quotes
```

For more information about TOML syntax, visit: https://toml.io/

## Supported Log Providers

ops-lens works with logs from any provider. Here are some common ones:

- **AWS CloudWatch** - Uses ERROR, WARN, INFO, DEBUG
- **Google Cloud Logging** - Uses ERROR, WARNING, INFO, DEBUG, NOTICE
- **Azure Monitor** - Uses ERROR, WARN, INFO, DEBUG
- **Datadog** - Uses ERR, WARN, INFO, DEBUG
- **Splunk** - Uses ERROR, WARN, INFO, DEBUG
- **ELK Stack** - Uses ERROR, WARN, INFO, DEBUG
- **Custom Applications** - Any keywords you define

## Future Features

Planned enhancements to the configuration system:
- Support for arrow keys and function keys in keybinds
- Custom color schemes and themes
- Log filtering and search
- Auto-scroll behavior customization
- Configuration profiles for different use cases
- Regular expression support for pattern matching

## Building from Source

### Requirements

- Rust 1.70 or later
- Cargo

### Build

```bash
cargo build --release
```

The executable will be available at `target/release/ops-lens`

## Dependencies

- **crossterm** - Terminal manipulation
- **ratatui** - Terminal UI framework
- **tokio** - Async runtime
- **serde** - Serialization framework
- **toml** - TOML configuration parsing

## License

See LICENSE file for details

## Support

For issues, feature requests, or contributions, please refer to the project documentation.
