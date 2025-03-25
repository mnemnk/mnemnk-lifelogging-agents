# Mnemnk Application Agent

`mnemnk-applicaiton` monitors the user's currently active application at regular intervals, excluding apps specified in the ignore list. It works as part of [Mnemnk app](https://github.com/mnemnk/mnemnk-app/) to help analyze user activity and visualize work patterns by integrating with other agents.

## Features

- Periodically detects the active window
- Ignores specified applications from logging

## Configuration Options

| Key        | Title               | Description                                                  | Type       | Default Value                          |
|------------|---------------------|--------------------------------------------------------------|------------|----------------------------------------|
| `interval` | Interval            | Interval (in seconds) between active window checks           | `integer`  | `10`                                   |
| `ignore`   | Ignore applications | List of application names to exclude from activity tracking  | `string[]` | `["LockApp.exe", "scrnsave.scr"]`      |

## License

MIT or Apache-2.0
