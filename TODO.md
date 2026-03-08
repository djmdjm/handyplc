TO-DO list

# DVT v0.1

Errors:

* Courtyard of U5 is reversed.
* Microswitches were not populated. Error in annotation or production file prep?
* Terminal block courtyards slightly wrong; too small on one side.
* Larger silkscreen on I/O; hard to read currently
* Mounting holes are much too small.
    * They do work ok for M2.5 screws, just not M3 as intended.

Changes / improvements:

* Reconsider I/O:
    * Some relay outputs. Omron p2r-05p sockets.
    * Some on-board MOSFETs
    * Piezo status buzzer
    * Reconsider mapping of GPIOs to outputs.
        * E.g. are TIMs mapped appropriately to inputs/outputs?
    * More high-speed logic-level outputs
        * Ideally enough for step + direction for 5 axes
        * Connect to TIM channels as much as possible
        * Better physical connector too (IDC header isn't great)
    * Dedicated FAULT LED for use by panic handler?
    * More LEDs generally
        * RGB?
    * headers / JST connectors for user buttons and LEDs to allow off-board use.
    * RS-422 IO?
    * Analog IO?
* Electrical:
    * Would be nice if inputs could be made bidi again.
    * Spare GPIO headers get +5VDC too
    * TVS diodes on everything.
    * Disconnect USBPWR when MCU externally powered
        * Manual or MOSFET
    * Provision for RTC battery
    * Fuse or PTC polyfuse
* Reconsider layout
    * Reconsider board dimensions / layout
    * More blank silkscreen annotation points.
    * Avoid long traces (there's plenty...)
    * Reconsider stackup
    * Isolation
    * Impedance matching of USB
    * General ergonomics
    * Mounting holes and pinout for cheap standard OLED screen.
    * Maybe mounting holes and pinout for general daugherboard stuff

Carrier board:

* Doesn't need to be as thick
* Properly skeletonise? (current hex skeletonisation is half-assed)
* Insert nuts instead of heat-set threads?
* Make properly parametric
