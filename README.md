# bleat_core

Originally forked from btleplug. This fork was created to fix outstaniding issues (namely the BLE device names not showing on Windows devices).

If you are looking for a rust BLE library I still recomend using [btleplug](https://github.com/deviceplug/btleplug), feel free to try this out, but it is currently being used for an internal project at might not have the same support btleplug has.

### Some things to note

- The no name on windows fix is done using a block_on call if the device name has not yet been set ([btle pull request #269](https://github.com/deviceplug/btleplug/pull/269) with some small improvements). This is not ideal asit can block the process up to roughly 120ms for unknown devices. However, if the device name has already been set it skips this check resuming 10uS < functionality (very crude bench tests). Additionally if the device name changes it will likely not be reflected for the duration of the running scan (possibly longer, I am still familiarising myself with the repository). 

## Platform Status

- **Linux / Windows / macOS / iOS / Android**
  - Device enumeration and characteristic/services implemented and working.
  - Please file bugs and missing features if you find them.
- **WASM/WebBluetooth**
  - WebBluetooth is possible, and a PR is in, but needs review.
  - [Tracking issue here](https://github.com/deviceplug/btleplug/issues/13)
  - Please hold off on filing more issues until base implementation is
    landed.


- X: Completed and released
- O: In development
- Blank: Not started

| Feature                               | Windows | MacOS / iOS | Linux | Android |
| ------------------------------------- | ------- | ----------- | ----- | ------- |
| Bring Up Adapter                      | X       | X           | X     | X       |
| Handle Multiple Adapters              |         |             | X     |         |
| Discover Devices                      | X       | X           | X     | X       |
| └ Discover Services                   | X       | X           | X     | X       |
| └ Discover Characteristics            | X       | X           | X     | X       |
| └ Discover Descriptors                | X       | X           | X     | X       |
| └ Discover Name                       | X       | X           | X     | X       |
| └ Discover Manufacturer Data          | X       | X           | X     | X       |
| └ Discover Service Data               | X       | X           | X     | X       |
| └ Discover MAC address                | X       |             | X     | X       |
| GATT Server Connect                   | X       | X           | X     | X       |
| GATT Server Connect Event             | X       | X           | X     | X       |
| GATT Server Disconnect                | X       | X           | X     | X       |
| GATT Server Disconnect Event          | X       | X           | X     | X       |
| Write to Characteristic               | X       | X           | X     | X       |
| Read from Characteristic              | X       | X           | X     | X       |
| Subscribe to Characteristic           | X       | X           | X     | X       |
| Unsubscribe from Characteristic       | X       | X           | X     | X       |
| Get Characteristic Notification Event | X       | X           | X     | X       |
| Read Descriptor                       | X       | X           | X     | X       |
| Write Descriptor                      | X       | X           | X     | X       | -->

## Library Features

#### Serialization/Deserialization

To enable implementation of serde's `Serialize` and `Deserialize` across some common types in the `api` module, use the `serde` feature.

```toml
[dependencies]
bleat_core = { version = "xx", features = ["serde"] }
```

## License

Bleat and the forked repository BTLEPlug is covered under a BSD 3-Clause License, with some parts from
Rumble/Blurmac covered under MIT/Apache dual license, and BSD 3-Clause
licenses, respectively. See LICENSE.md for more info and copyright
information. -->
