TO-DO list

# DVT v0.1

Errors:

* Courtyard of U5 is reversed.
* Microswitches were not populated. Error in annotation or production file prep?
* Terminal block courtyards slightly wrong; too small on one side.

Changes / improvements:

* Mounting holes are much too small.
* dedicated FAULT LED for use by panic handler?
* More LEDs generally
    * RGB?
* Mounting holes and pinout for cheap standard OLED screen.
* headers / JST connectors for user buttons and LEDs to allow off-board use.
* Spare GPIO headers get +5VDC too
* More blank silkscreen annotation points.
* TVS diodes on everything.
* More high-speed logic-level outputs
    * Ideally enough for step + direction for 5 axes
    * Connect to TIM channels as much as possible
* Reconsider mapping of GPIOs to outputs.
* Reconsider layout
    * Isolation
    * Avoid long traces
    * Impedance matching of USB
    * General ergonomics
* Reconsider stackup
* Disconnect USBPWR when MCU externally powered
    * Manual or MOSFET
* Provision for RTC battery
* Piezo status buzzer
* RS-422 IO?
* Analog IO?
* Maybe mounting holes and pinout for general daugherboard stuff
* Larger silkscreen on I/O
