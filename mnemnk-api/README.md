# Mnemnk API Agent

`mnemnk-api` is a [Mnemnk](https://github.com/mnemnk/mnemnk-app/) agent, which works as API server.

This API Agent is a component of the Mnemnk App, designed to expose external API functionality to Mnemnk app. It acts as a bridge that integrates external services into the Mnemnk environment, making them accessible within the app.

## Configuration

The agent supports the following configuration options:

### Address

Specifies the address of the API agent in the host:port format.

- Default: localhost:3296

### API Key

An optional API key used to authenticate incoming requests from the Mnemnk App. If provided, **you must share the same key with the client** to ensure successful communication.

- Default: ""

**For security reasons, never expose your API key in public.**

## Related Tools

### Mnemnk Browser Extenstion

This agent is used by the [Mnemnk Browser Extension](https://github.com/mnemnk/mnemnk-browser-extension).

The extension works in coordination with the API agent to share your browsing activity with the Mnemnk App, allowing Mnemnk app to make meaningful use of your web activity as part of its memory and context.

Make sure the extension is configured to point to your running API Agent instance (e.g., localhost:3296) and uses the correct API key if authentication is enabled.

## License

MIT or Apache-2.0
