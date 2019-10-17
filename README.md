# WMIDI

Midi encoding and decoding library suitable for real-time execution.

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/wmidi.svg)](https://crates.io/crates/wmidi)
[![docs.rs](https://docs.rs/wmidi/badge.svg)](https://docs.rs/wmidi)

# Changelog

## Planned 4.0

* Merge `MidiMessage::SysEx` and `MidiMessage::OwnedSysEx` into a single `Cow<u8>` backed variant.
* Remove `std::io::Read` trait from `MidiMessage`. Use `MidiMessage::copy_to_slice()` instead.
* Remove deprecated methods.

## 3.1-unreleased

* Rename `MidiMessage::wire_size()` to `MidiMessage::bytes_size()`.
* Introduce `MidiMessage::copy_to_slice()` method.

## 3.0

* Instances of U7 and U14 now have bounds checking.
* Note is now an enum instead of a u8. Can be converted with `Note::try_from` and `u8::from`.
