# Unplugged Mic Notification

🎤 | Notifies you when your microphone is (accidentally) unplugged

---

## Installation

Download the latest stable build using the side menu [(or by clicking here)](https://github.com/angeloanan/mic-unplugged-warning/releases).

If you live on the bleeding edge, you can also download the latest development build on [the actions tab](https://github.com/angeloanan/unplugged-mic-notification/actions).

## Building from scratch

You will need to have Rust installed. Specific to Linux, you will need to have `libasound2-dev` installed.

Run the following commands:

```sh
git clone https://github.com/angeloanan/unplugged-mic-notification.git
cd unplugged-mic-notification

cargo build --release
```
