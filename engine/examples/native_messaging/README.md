# Native Messaging Example

Validate [Native Messaging](https://developer.chrome.com/extensions/nativeMessaging) communication between browser extension and Stronghold

### usage
- `cargo build`
- open [chrome://extensions](chrome://extensions)
- enable "Developer mode" in the upper right
- click the "Load Unpacked" button, and load the "app" directory in this repo
- Find the ID that was assigned to your app, and make sure its included in the "allowed_origins" in `host/com.iota.stronghold.json`
- also in that file, edit the `path` to be the absolute path to the stronghold binary
- run `./host/register.sh`
- open a new tab in Chrome, click the "Apps" button on the top left, and open Stronghold Native Messaging Example app


