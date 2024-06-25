<div id="header" align="center">
  
<b>[Beacon Whisperer]</b>

(Covert Data Transmission via Bluetooth (without pairing, imitation by beacons)
</div>


<i><b>WARNING: This repository is for informational purposes only and should not be used for illegal activities.</b></i>

<i><b>WARNING: The project is not finalized (2022-2023), without support (simple MIT), created only to confirm the theory.</b></i>

<i><b>WARNING: The application was written back when esp32 did not have official support for bluetooth beacons on rust, so the code contained only links with C code.</b></i>


## Description

This repository contains a prototype system for covert data transmission via Bluetooth, mimicking beacons. The system is developed for educational purposes and is intended for experimentation.

## Key Features

• ESP32 (Rust): The ESP32 microcontroller, programmed in Rust, emulates a standard Bluetooth Low Energy (BLE) beacon and sends hidden data.

• Android (DemoApp, Cordova, JS): A demo Android application, developed using Cordova and JavaScript, demonstrates the sending and receiving of hidden data.

## Protocol Features:

• Data Encoding: The UUID and ID fields of iBeacons are utilized to encode data. Partial UUIDs and ID omission are supported, offering flexibility in data representation.

• Carousel Transmission: Beacons are transmitted in a carousel manner with a defined interval, ensuring reliable data delivery.

• Complete Transmission: Transfer is considered successful only after all beacons have been received.

• Bidirectional Communication: The system supports both data input and output. Upon successful transmission, the device simulates an iBeacon, allowing for bidirectional communication.

• Flexible Data Size: Supports any number of data bytes, prioritizing data queue management.

• Data Security: Data is compressed and encrypted for enhanced security.

• Queue Management & Multi-Device Support: Implements robust queuing mechanisms to handle multiple data streams and allows for simultaneous data transmission and reception from multiple devices.


## Repository Structure

• esp32 (Rust)

• demo_android_bluetest (Android, JS, Cordova)

## License

Copyright 2022-2023 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the MIT License.

<b>This project is provided as is without warranty of any kind.</b>
