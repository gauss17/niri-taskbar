# Niri Taskbar (for Waybar)

This provides a combined [Waybar][waybar] taskbar + workspace indicator for [Niri][niri].

Displays ordered list of workspaces, each with a list of its windwows ordered by
workspace position (floating windows last). Current workspace and focused window
are highlighted.

![Example screenshot](images/screenshot.png)

## Installation

At the moment, this needs to built from source. (If you do package it for a
distro, let me know and I'll update the README.)

### Requirements

- Rust 1.87.0 or later
- Niri 25.08
- Gtk+ 3 (including the development package on distros that separate those out)
- Waybar 0.12.0 (or any version that's API compatible with 0.12, which will
  _probably_ include later versions, but I have no actual knowledge there)

### Building

The standard Rust build process should work fine:

```bash
$ cargo build --release
```

This will give you a shared library module at
`target/release/libniri_taskbar.so`. Feel free to move that wherever makes
sense.

## Configuration

This uses the normal configuration for a [CFFI Waybar module][cffi], which in
practice will look something like this:

```jsonc
{
  "modules_left": ["cffi/niri-taskbar"],
  // ...
  "cffi/niri-taskbar": {
    "module_path": "/your/path/to/libniri_taskbar.so",
    "orientation": "vertical",
    "workspace_format": "󱋰󱋰",
    "workspace_format_focused": "󱋰󱋰"
  }
}
```

### Application highlighting

In addition to [notification support](#notifications), you can highlight
applications based on their app ID and title by configuring application rules in
the Waybar configuration.

For example, to highlight a Signal window that starts with `(1)` (or any other
number), indicating pending notifications, you could configure the taskbar like
so:

```jsonc
{
  "cffi/niri-taskbar": {
    // module_path
    "apps": {
      "signal": [
        {
          "match": "\\([0-9]+\\)$",
          "class": "unread",
        },
      ],
    },
  },
}
```

Each key within the `apps` object is a Wayland app ID, which can have one or
more rules set within it. Each rule must have a `match`, which is a regex that
will be matched against the window title, and a `class`, which is a CSS class
that will be added to the button element if the regex matches.

If more than one rule matches for a single app ID, all matching classes will be
added.

The easiest way to get the app ID for a window is to ask Niri with `niri msg
windows`. Note that app IDs are case sensitive.

### Multiple outputs

By default, the taskbar will only show applications running on the same output
as the taskbar itself. You can enable the `show_all_outputs` option to show all
applications on all outputs:

```jsonc
{
  "cffi/niri-taskbar": {
    // other settings
    "show_all_outputs": true
  }
}
```

Note that multiple output support is currently experimental, and may have some
quirks. Please open an issue with your use case if it's not working as you
expect!

### Notifications

You can enable the `notifications` configuration option to have the taskbar
listen to notifications and attempt to highlight the app that sent the
notification.

Configuration wise:

```jsonc
{
  "cffi/niri-taskbar": {
    // other settings
    "notifications": true,
  },
}
```

Highlighted buttons will gain the `.urgent` CSS class. Default styling is
included, but can be overridden [as described below](#styling).

## Styling

The taskbar uses [the same Gtk styling mechanism as Waybar][style]. The top
level taskbar element is given the class `.niri-taskbar`, and contains box
containers of class `.niri-workspace` for each workspaces (the current one
gets class `.niri-workspace_focused`).
Inside each workspace container are `button` elements for each window. The
only CSS class that is applied by default is the `focused` class, which is
added to the button for the currently focused window.

The default styling assumes a dark background. It provides a basic hover
effect, and highlights the focused window.

For a light background, something like this is likely good:

```css
.niri-taskbar button:hover {
  background: rgba(0, 0, 0, 0.5);
}

.niri-taskbar button.focused {
  background: rgba(0, 0, 0, 0.3);
}
```

For workspace highlight, something like this:

```css
.niri-workspace {
  border: 2px solid transparent;
  background: transparent;
}

.niri-workspace-focused {
  border: 2px solid rgba(255, 0, 255, 0.2);
  background: rgba(255, 0, 255, 0.1);
}
```

If you apply custom CSS classes using application rules as described above,
then those can be styled in the same way. For instance, with the `unread` class
demonstrated above, you could add a border highlight like so:

```css
.niri-taskbar button {
  /* This is useful to prevent the icon being resized when there's no unread. */
  border-bottom: solid 3px transparent;
}

.niri-taskbar button.unread {
  border-bottom: solid 3px white;
}
```

[cffi]: https://github.com/Alexays/Waybar/wiki/Module:-CFFI
[niri]: https://github.com/YaLTeR/niri
[style]: https://github.com/Alexays/Waybar/wiki/Styling
[waybar]: https://github.com/Alexays/Waybar
