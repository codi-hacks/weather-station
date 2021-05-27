[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE.md)

This is a monorepo with all the instructions and software to build and program your own solar-powered wi-fi weather station(s).

- [Overview](#overview)
- [Building the weather stations](hardware-files/README.md)
- [Arduino sketch README](sketch/README.md)
- [API server README](server/README.md)
- [Web app README](web-app/README.md)

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
    - [ ] Introduce [Serde](https://github.com/serde-rs/serde)
    - [ ] Setting up relationships ([#11](https://github.com/JTCC-Programming-Club/weather-station/issues/11), continued)
  - Web app workshop (part 5)
    - [ ] Fetching, storing, and displaying server data ([#7](https://github.com/JTCC-Programming-Club/weather-station/issues/7))
    - [ ] Create Station page using responsive flexbox cards ([#8](https://github.com/JTCC-Programming-Club/weather-station/issues/8))
- **Week 11**
  - Web app workshop (part 6)
    - [ ] Graphing data inside the cards
    - [ ] Add time series filter (view weather over period of 1yr, 1mo, 1wk, 1dy) ([#19](https://github.com/JTCC-Programming-Club/weather-station/issues/19))
    - [ ] Display current and average metrics
- **Future**
  - [ ] Customizing the PWA mobile experience
  - [ ] Add latitude/longitude display?
