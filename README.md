<div align="center">
  <h3 align="center">Mnemnk Lifelogging Agents</h3>

![Badge Language] 
[![Badge License]][License]

<a target="_blank" href="https://github.com/mnemnk/mnemnk-lifelogging-agents/blob/main/docs/img/screenshot-lifelogging-agents.png?raw=true"><img alt="lifelogging agents" width="60%" src="https://github.com/mnemnk/mnemnk-lifelogging-agents/blob/main/docs/img/screenshot-lifelogging-agents.png?raw=true"></a>

</div>

## Getting Started

### Prerequisites

- [Mnemnk App](https://github.com/mnemnk/mnemnk-app)
- [Rust](https://www.rust-lang.org/)

### Installation

0. Install the [Mnemnk App](https://github.com/mnemnk/mnemnk-app).

    When you launch the Mnemnk App, the settings page will open, prompting you to configure the Mnemnk Directory. After configuration, restart the Mnemnk App, and a directory named `agents` will be created in the Mnemnk Directory.

1. Clone the repo into the `{Mnemnk Directory}/agents/`

    ```shell
    cd {Mnemnk Directory}/agents/
    git clone https://github.com/mnemnk/mnemnk-lifelogging-agents.git
    ```

2. Build

    ```shell
    cd mnemnk-lifelogging-agents
    cargo build --release
    ```

### Usage

Open the Agents page in Mnemnk App, and press the `Agents` button to display the list of agents. Confirm that `API`, `Application`, and `Screen` have been added under "Lifelogging" category.

Import [logging.json](https://github.com/mnemnk/mnemnk-lifelogging-agents/blob/main/logging.json) under the cloned directory via File > Import menu on the Agents page.

<a target="_blank" href="https://github.com/mnemnk/mnemnk-lifelogging-agents/blob/main/docs/img/screenshot-lifelogging-agents-imported.png?raw=true"><img alt="lifelogging agents" width="60%" src="https://github.com/mnemnk/mnemnk-lifelogging-agents/blob/main/docs/img/screenshot-lifelogging-agents-imported.png?raw=true"></a>

The flow will be imported in a stopped state (for security reasons). Press the play button at the bottom of the screen to start it.

Save the flow via File > Save.

Open Home page, and confirm today's date is highlighted. Click it to view the records.

## Acknowledgments

* mnemnk-api
  * [axum](https://github.com/tokio-rs/axum)
  * [tower](https://github.com/tower-rs/tower)
* mnemnk-application
  * [active-win-pos-rs](https://github.com/dimusic/active-win-pos-rs)
* mnemnk-screen
  * [xcap](https://github.com/nashaofu/xcap)

<!----------------------------------------------------------------------------->

[License]: https://github.com/mnemnk/mnemnk-lifelogging-agents

<!----------------------------------{ Badges }--------------------------------->

[Badge Language]: https://img.shields.io/github/languages/top/mnemnk/mnemnk-lifelogging-agents
[Badge License]: https://img.shields.io/badge/License-MIT%20or%20Apache%202-green.svg
