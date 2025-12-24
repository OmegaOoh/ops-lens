# ops-lens Configuration Guide

## Overview

ops-lens uses a TOML configuration file to customize keybinds and log level display settings. By default, the application looks for a configuration file named `ops-lens.toml` in the current working directory.

## Configuration File Location

The configuration file should be placed in the directory where you run the application:
```
ops-lens.toml
```

If the file doesn't exist, ops-lens will automatically create one with default settings on first run.

## Configuration Structure

### Keybinds Section

The `[keybinds]` section defines keyboard shortcuts for controlling the application.

```toml
[keybinds]
quit = "q"              # Quit the application
edit = "e"              # Edit the log file path
scroll_up = "k"         # Scroll up in the log view
scroll_down = "j"       # Scroll down in the log view
```

#### Available Keybind Commands

- `quit`: Exit the application
- `edit`: Enter edit mode to change the log file path
- `scroll_up`: Scroll up in the log viewer
- `scroll_down`: Scroll down in the log viewer

#### Customizing Keybinds

You can customize any keybind by changing the corresponding value to a single character:

```toml
[keybinds]
quit = "Q"              # Use Shift+Q to quit
edit = "E"              # Use Shift+E to edit
scroll_up = "w"         # Use 'w' to scroll up
scroll_down = "s"       # Use 's' to scroll down
```

**Supported Keys:**
- Single characters: `a-z`, `A-Z`, `0-9`
- Special keys will be supported in future versions

### Log Level Section

The `[log_level]` section controls how log level information is displayed in the UI.

```toml
[log_level]
enabled = true                      # Enable/disable log level indicator
indicator_position = "top"          # Position: "top" or "bottom"
show_level_counts = true            # Show count of each log level
```

#### Log Level Configuration Options

- `enabled`: Boolean (true/false) - Enable or disable the log level indicator display
- `indicator_position`: String ("top" or "bottom") - Where to show the log level indicator in the UI
- `show_level_counts`: Boolean (true/false) - Display the count of each log level type

#### Example Configurations

**Minimal Configuration (Log Level Disabled):**
```toml
[log_level]
enabled = false
```

**Top Indicator with Counts:**
```toml
[log_level]
enabled = true
indicator_position = "top"
show_level_counts = true
```

**Bottom Indicator without Counts:**
```toml
[log_level]
enabled = true
indicator_position = "bottom"
show_level_counts = false
```

## Default Configuration

If no configuration file exists, ops-lens will create one with these defaults:

```toml
# ops-lens Configuration File
# This file contains settings for keybinds and log level display

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
```

## Examples

### Example 1: Vi-like Navigation

If you prefer vi-style keybindings:

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
```

### Example 2: Arrow Key Alternative

Using alternative keybinds:

```toml
[keybinds]
quit = "q"
edit = "e"
scroll_up = "w"
scroll_down = "s"

[log_level]
enabled = true
indicator_position = "bottom"
show_level_counts = false
```

### Example 3: Minimal UI

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

## Troubleshooting

### Configuration File Not Being Read

1. **Check file location**: Ensure `ops-lens.toml` is in the same directory where you run the application
2. **Check file permissions**: Make sure the file is readable
3. **Check TOML syntax**: Ensure the TOML file is properly formatted. Use a TOML validator if unsure
4. **Check for error messages**: ops-lens will print warnings if it fails to load the config file

### Invalid Configuration Errors

If the configuration file has syntax errors:
- ops-lens will print a warning message indicating the parse error
- The application will fall back to default settings
- Review the error message to identify the issue

### Changes Not Taking Effect

- Changes to the configuration file require a restart of the application
- Make sure to save the file after editing

## TOML Syntax Quick Reference

```toml
# Comments start with #

[section_name]
# Key-value pairs use = assignment
key = "value"
boolean_key = true
number_key = 42

# Strings must be quoted
string_key = "this is a string"
```

For more information about TOML, visit: https://toml.io/

## Future Features

Planned enhancements to the configuration system:
- Support for arrow keys and function keys in keybinds
- Custom color schemes
- Log filtering rules
- Auto-scroll behavior customization
- Theme support