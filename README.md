<div align="center">
  <h3 align="center">Mnemnk Core Agents</h3>

![Badge Language] 
[![Badge License]][License]

<a target="_blank" href="https://github.com/mnemnk/mnemnk-core-agents/blob/main/docs/img/screenshot-core-agents.png?raw=true"><img alt="core agents" width="60%" src="https://github.com/mnemnk/mnemnk-core-agents/blob/main/docs/img/screenshot-core-agents.png?raw=true"></a>

</div>

## Getting Started

### Prerequisites

- [Mnemnk App](https://github.com/mnemnk/mnemnk-app)
- [Rust](https://www.rust-lang.org/)

### Installation

0. Install the [Mnemnk App](https://github.com/mnemnk/mnemnk-app).

When you launch the Mnemnk App, the settings page will open, prompting you to configure the Mnemnk Directory. After configuration, restart the Mnemnk App, and a directory named `agents` will be created in the Mnemnk Directory.

1. Clone the repo into the `{Mnemnk Diretory}/agents/`

```shell
cd {Mnemnk Diretory}/agents/
git clone https://github.com/mnemnk/mnemnk-core-agents.git
```

2. Build

```shell
cargo build --release
```

3. Restart the Mnemnk App.

Open the Agents page and press the `Agents` button to display the list of agents. Confirm that `API`, `Application`, and `Screen` have been added under Core.

4. Import the Flow

Download [logging.json](https://github.com/mnemnk/mnemnk-core-agents/blob/main/logging.json?raw=true) and import it via File > Import on the Agents page.

<a target="_blank" href="https://github.com/mnemnk/mnemnk-core-agents/blob/main/docs/img/screenshot-core-agents-imported.png?raw=true"><img alt="core agents" width="60%" src="https://github.com/mnemnk/mnemnk-core-agents/blob/main/docs/img/screenshot-core-agents-imported.png?raw=true"></a>

The flow will be imported in a stopped state (for security reasons). Press the play button at the bottom of the screen to start it.

Save the flow via File > Save.

5. Return to Home

Confirm today's date is highlighted. Click it to view the records.

<!----------------------------------------------------------------------------->

[License]: https://github.com/mnemnk/mnemnk-core-agents

<!----------------------------------{ Badges }--------------------------------->

[Badge Language]: https://img.shields.io/github/languages/top/mnemnk/mnemnk-core-agents
[Badge License]: https://img.shields.io/badge/License-MIT%20or%20Apache%202-green.svg
