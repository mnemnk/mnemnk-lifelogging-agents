# Mnemnk Screen Agent

`mnemnk-screen` captures periodic screenshots of the user's screen. It works as part of [Mnemnk app](https://github.com/mnemnk/mnemnk-app/) to monitor visual activity, track changes, and assist with documentation or review purposes by integrating with other agents.

## Use Cases

This agent is useful for:

- Automatically logging visual activity over time
- Capturing screen changes to analyze user behavior or workflows

## Features

- Takes screenshots at regular intervals
- Detects screen changes to avoid saving duplicates
- Saves only non-blank screens

## Default Configuration

The following parameters are available in the `default_config`:

| Parameter Name            | Default Value | Type     | Description |
|---------------------------|----------------|----------|-------------|
| `interval`                | 60             | integer  | Time interval (in seconds) between screenshots |
| `almost_black_threshold`  | 20             | integer  | Each RGB value is considered black if below this threshold |
| `non_blank_threshold`     | 400            | integer  | Number of non-black pixels required to consider a screen non-blank |
| `same_screen_ratio`       | 0.01           | number   | If the ratio of different pixels is below this value, the screen is considered unchanged |

## Notes

- It is currently assumed that desktop and lock screens are black, though this behavior may be made more flexible in the future.

## License

MIT or Apache-2.0
