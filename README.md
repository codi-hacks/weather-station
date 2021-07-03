[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE.md)

This is a [monorepo](https://en.wikipedia.org/wiki/Monorepo) with all the instructions and software to build and program your own solar-powered wi-fi weather station(s).

- [Overview](#overview)
- [Building the weather stations](hardware-files/README.md)
- [Arduino sketch README](sketch/README.md)
- [API server README](server/README.md)
- [Web app README](web-app/README.md)

| | |
|:-------------------------:|:-------------------------:|
| [![Generic-enclosure station](/hardware-files/station1.jpg)](hardware-files/station1.jpg) | [![Generic-enclosure station](/hardware-files/station2.jpg)](hardware-files/station2.jpg) |
| [![3D-printed station](/hardware-files/station3.jpg)](hardware-files/station3.jpg) | [![3D-printed station](/hardware-files/station4.jpg)](hardware-files/station4.jpg) |

## Overview

The weather stations send measurements to the API server to collect, which in turn serves this data to the web app that users load on their computer or mobile device.

![diagram overviewing the software architecture](./diagram.svg)

<!-- (Flowchart generated using [gojs](https://gojs.net/latest/samples/flowchart.html). Load `diagram.json` onto their site to generate a new svg.) -->

**Folder structure:**

- **[hardware-files/](hardware-files/)** - All the files needed to print and build the weather station hardware
- **[sketch/](sketch/)** - Embedded software that runs on the weather stations
- **[server/](server/)** - API server
- **[web-app/](web-app/)** - Website/mobile app

**Milestones:**

- **Week 1**
  - [x] Project introduction
  - [x] Explanation of architecture
  - [x] Nodejs review
- **Week 2**
  - [x] API introduction
  - [x] fetching data in the web app
- **Week 3**
  - [x] Discussing Rust
  - [x] Database introduction
  - Web app workshop (part 1)
    - [x] Set up [node.js](https://nodejs.org)
- **Week 4**
  - ESP8266 Workshop (part 1)
    - [x] Hardware and Arduino introduction
    - [x] [Sketch 1: Hello World](https://gist.github.com/jaythomas/69a7bacf49e3f26ae8311a25ec416702)
  - Web app workshop (part 2)
    - [x] Import a charting library for displaying data ([#1](https://github.com/JTCC-Programming-Club/weather-station/issues/1))
    - [x] Discuss creating components
- **Week 5**
  - ESP8266 Workshop (part 2)
    - [x] [Sketch 2: LED Blink](https://gist.github.com/jaythomas/5bc647d795368d76fbcd233d69ee4246)
    - [x] [Sketch 3: LED Swell](https://gist.github.com/jaythomas/56faf188e171e11e31d73bcf0457b042)
    - [x] [Sketch 4: RGB LED](https://gist.github.com/jaythomas/2163c926c71bd153f35c89ae6f34b350)
    - [x] [Sketch 5: RGB Swell](https://gist.github.com/jaythomas/4e1c2e71ac708f6263b3ec3324602426)
    - [x] [Sketch 6: Wifi](https://gist.github.com/jaythomas/c6a8850c13ec2fddc878c8dadebfae91)
    - [x] [Sketch 7: FINAL BOSS](https://gist.github.com/jaythomas/0f9becea61da928d38879eb3563897fa)
- **Week 6**
  - [x] Data modeling on the API versus in the web app
- **Week 7**
  - Web app workshop (part 3)
    - [x] Routing ([#2](https://github.com/JTCC-Programming-Club/weather-station/issues/2))
- **Week 8**
  - API server workshop (part 1)
    - [x] Set up [PostgreSQL](https://www.postgresql.org/)
    - [x] Getting started with [Diesel](http://diesel.rs/)
- **Week 9**
  - API server workshop (part 2)
    - [x] Introduce [Actix/Actix Web](https://actix.rs/)
    - [x] Create database schema and server resources ([#11](https://github.com/JTCC-Programming-Club/weather-station/issues/11))
  - Web app workshop (part 4)
    - [x] Discuss GitHub Issues and Milestones
    - [x] Discuss props and v-model ([#6](https://github.com/JTCC-Programming-Club/weather-station/issues/6))
- **Week 10**
  - API server workshop (part 3)
    - [x] Introduce [Serde](https://github.com/serde-rs/serde)
    - [x] Setting up relationships ([#11](https://github.com/JTCC-Programming-Club/weather-station/issues/11), continued)
  - Web app workshop (part 5)
    - [x] Fetching, storing, and displaying server data ([#7](https://github.com/JTCC-Programming-Club/weather-station/issues/7))
    - [x] Create Station page using responsive flexbox cards ([#8](https://github.com/JTCC-Programming-Club/weather-station/issues/8))
- **Week 11**
  - Web app workshop (part 6)
    - [x] Graphing data inside the cards
    - [x] Add time series filter (view weather over period of 1 days, 2 days, 1 week, 90 days) ([#19](https://github.com/JTCC-Programming-Club/weather-station/issues/19))
  - API server workshop (part 4)
    - [x] Returning nested data
- **Week 12**
  - API server workshop (part 5)
    - [x] Add UDP socket and record some real measurements ([#12](https://github.com/JTCC-Programming-Club/weather-station/issues/12))
  - Web app workshop (part 7)
    - [x] Add card "modes" ([#31](https://github.com/JTCC-Programming-Club/weather-station/pull/31))
    - [x] Display current and average metrics ([#31](https://github.com/JTCC-Programming-Club/weather-station/pull/31))
- **Week 13**
  - API server workshop (part 6)
    - [ ] Update middleware ([#15](https://github.com/JTCC-Programming-Club/weather-station/issues/15))([#16](https://github.com/JTCC-Programming-Club/weather-station/issues/16))
    - [ ] Clean up routes ([#26](https://github.com/JTCC-Programming-Club/weather-station/issues/26))
    - [ ] Add station cli script ([#14](https://github.com/JTCC-Programming-Club/weather-station/issues/14))
  - Web app workshop (part 7)
    - [ ] Add button to zoom reset button ([#32](https://github.com/JTCC-Programming-Club/weather-station/issues/32))
    - [ ] Create dashboard ([#9](https://github.com/JTCC-Programming-Club/weather-station/issues/9))
- **Week 14**
  - API server workshop (part 7)
    - [ ] Add homepage ([#21](https://github.com/JTCC-Programming-Club/weather-station/issues/21))
    - [ ] Connect to server from web app
  - Web app workshop (part 8)
    - [ ] Customizing the PWA mobile experience (color themes, service worker functionality)
    - [ ] Add dashboard prompt when dashboard is empty ([#3](https://github.com/JTCC-Programming-Club/weather-station/issues/3))
- **Week 15**
  - **Wrapping up final issues...**
