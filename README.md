# balenaFin SDK

Multi-language SDK for the [balenaFin professional carrier board](https://www.balena.io/fin/) for the Raspberry Pi Compute Module.

[**Python**](#python) | [**JavaScript**](#javascript) | [**API**](#api) | [**License**](#license)

## Python

For installation and usage under balenaOS please refer to the [Python example application](./examples/py/properties).

## JavaScript

For installation and usage under balenaOS please refer to the [JavaScript example application](./examples/js/properties).

## API

**Fin.Client.revision**

balenaFin hardware revision. For balenaFin v1.0 boards the result will be `09` and for v1.0 boards it will be `10`, which corresponds to the revision number on the board's QR code.

**Fin.Client.eeprom***

balenaFin raw EEPROM data.

**Fin.Client.uid**

Board's unique ID.

## License

Copyright 2019 Balena Ltd.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

<http://www.apache.org/licenses/LICENSE-2.0>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
