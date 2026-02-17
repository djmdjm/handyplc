# HandyPLC

This is a small PLC-like device based around a STM32F411 microcontroller.
It mostly exists because I wanted a PLC that I could program in Rust
and without vendor toolchains or closed-source libraries.

## Features and specifications

As the name suggests, HandyPLC is fairly general-purpose.

* Power supply: 24VDC nominal, 2A (worst-case). MCU power is isolated.
* Inputs: 16x isolated inputs. 5-36V
* General-purpose outputs: 16x isolated open-drain outputs. Can sink 33mA at 24V
* High-speed outputs: 4x isolated 5V logic outputs. Max 10mA.
* 3x signalling LEDs
* 3x micro-switches for UI
* USB-C interface (USB 2.0 only). Can be used for DFU.
* Spare GPIO brough out to IDC connectors and/or headers.

## Firmware

There is no standard firmware for this device. The intent is that you fork
this repository and write your own. The `firmware/` sub-directory contains
the Rust firmware that I use, but that is completely specific for my needs.

## Mechanical

The `carrier/` subdirectory contains CAD files for the holder/carrier that
I used to attach a HandyPLC to a DIN rail.

## Bugs

Probably plenty. I'm not an Electrical Engineer and have no training or
experience designing industrial devices.

Notice in particular that I have no provided any specification around the
isolation properties of this device.

Use this completely at your own risk.

## History

2026/02 - initial release (DVT 0.1)
