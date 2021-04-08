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

- [x] Week 1
  - [x] Project introduction
  - [x] Explanation of architecture
  - [x] Nodejs review
- [x] Week 2
  - [x] API introduction
  - [x] fetching data in the web app
- [x] Week 3
  - [x] Discussing Rust
  - [x] Database introduction
- [ ] Week 4
  - [ ] Hardware and Arduino introduction
  - [ ] Try out the sketches:
    - [ ] [Sketch 1: Hello World](https://gist.github.com/jaythomas/69a7bacf49e3f26ae8311a25ec416702)
    - [ ] [Sketch 2: LED Blink](https://gist.github.com/jaythomas/5bc647d795368d76fbcd233d69ee4246)
    - [ ] [Sketch 3: LED Swell](https://gist.github.com/jaythomas/56faf188e171e11e31d73bcf0457b042)
    - [ ] [Sketch 4: RGB LED](https://gist.github.com/jaythomas/2163c926c71bd153f35c89ae6f34b350)
    - [ ] [Sketch 5: RGB Swell](https://gist.github.com/jaythomas/4e1c2e71ac708f6263b3ec3324602426)
- [ ] Web app workshop
- [ ] API server workshop
- [ ] Arduino workshop
- [ ] Deploying everything to the real world
- Optional features
  - [ ] Make app installable and full screen on mobile ([PWA demo](https://youtu.be/S7TIVG5F2xw))
  - [ ] Add time series filter (view weather over period of 1yr, 1mo, 1wk, 1dy)
  - [ ] Add latitude/longitude display
