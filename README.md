# stm32f411re-embedded-rust-sgp30-air-quality-sensor

Quick Pinout Guide
------

| Nucleo-F411RE Pin | SGP30 Pin   |
|-------------------|-------------|
| 5V                | Vin/5V      |
| GND               | GND         |
| PB8               | SCL         |
| PB9               | SDA         |


What is this project?
------

This is based on the [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart) project.

The code is for the stm32-f411re processor, and is meant for the nucleo-f411re board. This is for the SGP30 air quality sensor. This will display the current CO2 level and total volatile organic component levels after a 15 second startup period.

I've included some resources that have helped me along the way.

![Nucleo F411RE Alternate Function Mappings](/alternate-function-mappings-p2.png)

![Arduino Connectors Part 1](/arduino-connectors-p1.png)

![Arduino Connectors Part 2](/arduino-connectors-p2.png)

![Nucleo F411RE Mappings](/nucleo-f411re-mappings.png)
