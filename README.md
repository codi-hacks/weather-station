This is a monorepo with all the instructions and software to build and program your own solar-powered wi-fi weather station(s).

- [Overview](#overview)
- [Building the weather stations](hardware-files/README.md))
- [Arduino sketch README](sketch/README.md)
- [API server README](server/README.md)
- [Web app README](web-app/README.md)

## Overview

The weather stations send measurements to the API server to collect, which in turn serves this data to the web app that users load on their computer or mobile device.

![diagram overviewing the software architecture](./diagram.svg)

<!-- (Flowchart generated using [gojs](https://gojs.net/latest/samples/flowchart.html). Load `diagram.json` onto their site to generate a new svg.) -->

**Milestones:**

- [x] Week 1
  - [ ] Project introduction
  - [ ] Explanation of architecture
  - [ ] Nodejs review
- [ ] Week 2
  - [ ] API introduction
- [ ] Hardware and Arduino introduction
- [ ] Web app workshop
- [ ] API server workshop
- [ ] Arduino workshop
- [ ] Deploying everything to the real world
- Optional features
  - [ ] Make app installable and full screen on mobile ([PWA demo](https://youtu.be/S7TIVG5F2xw))
  - [ ] Add time series filter (view weather over period of 1yr, 1mo, 1wk, 1dy)
  - [ ] Add latitude/longitude display
