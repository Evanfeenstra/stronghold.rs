# Native Messaging Example

Validate [Native Messaging](https://developer.chrome.com/extensions/nativeMessaging) communication between browser extension and Stronghold

### usage
- cd engine/examples/native_messaging
- `cargo build`
- in `host/com.iota.stronghold.json` edit the `path` to be the absolute path to the binary 
- run `./host/register.sh`
- open [chrome://extensions](chrome://extensions)
- enable "dev mode" in the upper right
- click the "Load Unpacked" button
- load the "app" directory in this repo
- open a new tab in Chrome, click the "Apps" button on the top left, and open Stronghold Native Messaging Example app


