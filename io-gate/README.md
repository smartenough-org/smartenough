IO-Gate - Computer side
=======================

This is a part of a `CAN <-> Home Assistant` bridge that is required to control
the `io-ctrl` boards from mobile phones, computers, etc.

Example communication schema:

           Distributed and autonomous IO controlling boards:

           +-----------+   +-----------+   +-----------+
           |           |   |           |   |           |
           |  IO Ctrl  |   |  IO Ctrl  |   |  IO Ctrl  |
           |           |   |           |   |           |
           +-----------+   +-----------+   +-----------+
                ||               ||              ||
    CAN Bus:    ||               ||              ||  (termination)
           ++---++---------------++--------------++
           ++---++---------------++--------------++
           ||
           || (termination)     PC, VM, Raspberrypi:
    +-------------+           +-----------------------+
    |             |           |                       |
    |   CAN-USB   |<---USB--->|    IO-Gate Daemon     |
    |   Bridge    |           |                       |
    |             |           |    (Your are here)    |
    +-------------+           +-----------------------+
                                         |
                                         |  LAN / Ethernet
                                         |
       +----------------+         +-------------+
       |                |   LAN   |             |
       | Home Assistant |<------->|    MQTT     |
       |                |         |             |
       +----------------+         +-------------+


- `io-ctrl` boards communicate using a `CAN` bus interface. CAN should be
  terminated using 120 ohm resistors at the ends of a bus.
  They use `io-ctrl` firmware and a `ctrl` binary.
- One board connects the `CAN` to the computer network (`io-ctrl` firmware with
  a `gate` binary).
- `io-gate` daemon communicates with CAN bridge over USB and generates Home
  Assistant compatible messages for the MQTT bus.
- The `io-gate`, Home Assistant and MQTT (usually mosquitto) can share the same
  computer (or a virtual machine).
- High-availability: There can be multiple bridges but only single daemon should
  be running at a time.
