# Alnico

**A competitor to JUCE for Rust** — the whole path from "I have a filter" to
"signed `.vst3` that loads in Logic", built on what audio development actually
looks like in 2026, and permissively licensed.

> ⚠️ **This release reserves the name. Nothing is implemented yet.**
> Development is tracked at <https://github.com/NotTheDj/alnico-audio>.

*Alnico* — aluminium-nickel-cobalt, the magnet alloy in speaker drivers and
guitar pickups.

## What it will be

- **DSP** — filters, FFT, convolution, oversampling, dynamics, analysis
- **Device I/O** — CoreAudio, WASAPI, ASIO, ALSA, JACK, PipeWire; CoreMIDI,
  Windows MIDI Services, ALSA seq
- **Plugin export** — CLAP-native, with VST3, AU and standalone as adapters
- **GUI** — our own retained toolkit on our own GPU and CPU rasterizers
- **The development loop** — hot-reloadable DSP, a harness that runs your
  processor with no DAW anywhere, golden-audio and property testing, and one
  command to sign, notarize and package

The wedge is developer experience. Today, changing one line of DSP means
recompile, relaunch the DAW, rescan, reload, re-navigate, re-trigger. That is
the problem this project exists to delete.

## Status

Pre-alpha, solo-developed, no usable code yet. See the
[development plan](https://github.com/NotTheDj/alnico-audio) for scope,
architecture decisions and roadmap.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this work by you, as defined in the Apache-2.0 licence, shall
be dual licensed as above, without any additional terms or conditions.
