# keebgrab ⌨️🔍

`keebgrab` is a simple keylogger for X11 based desktops.

## Features

* No root required.
* No dependencies (like `xinput`).
* Instant conversion of keycodes to keys.
* (TBD) raw mode (key press and release) and text mode (plaintext)

## Non features

* Mouse movements and buttons are not logged.
* [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) is not supported. While not completely immune to keyloggers, it offers much better event isolation.

## License

[MIT License](./LICENSE)

## Disclaimer

This is a proof of concept with strictly educational and debugging purposes. Using a keylogger without consent for identity or credential theft, stalking, or any malicious intent in general is illegal.