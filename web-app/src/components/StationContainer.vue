<template>
  <div class="flex-container">
    <v-card
      class="flex-item"
      rounded
      v-for="sensor in stationSensors"
      :key="sensor.id">
      <ElevationCard v-if="sensor.type.label === 'elevation'" :sensor="sensor" />
      <HumidityCard v-else-if="sensor.type.label === 'humidity'" :sensor="sensor" />
      <PressureCard v-else-if="sensor.type.label === 'pressure'" :sensor="sensor" />
      <SignalCard v-else-if="sensor.type.label === 'signal'" :sensor="sensor" />
      <TemperatureCard v-else-if="sensor.type.label === 'temperature'" :sensor="sensor" />
      <VoltageCard v-else-if="sensor.type.label === 'voltage'" :sensor="sensor" />
      <div v-else>
        Invalid sensor type!
      </div>
    </v-card>
  </div>
</template>

<script>
import ElevationCard from './cards/Elevation'
import HumidityCard from './cards/Humidity'
import PressureCard from './cards/Pressure'
import SignalCard from './cards/Signal'
import TemperatureCard from './cards/Temperature'
import VoltageCard from './cards/Voltage'

export default {
  components: {
    ElevationCard,
    HumidityCard,
    PressureCard,
    SignalCard,
    TemperatureCard,
    VoltageCard
  },
  props: {
    station: {
      required: true,
      type: Object
    },
    sensors: {
      required: true,
      type: Object
    }
  },
  computed: {
    stationSensors() {
      return this.station.sensors
        .map(s => this.sensors[s.id])
        .filter(s => s)
    }
  }
}
</script>

<style scoped>
.flex-container {
  display: flex;
  flex-flow: row wrap;
  justify-content: space-around;
}

.flex-item {
  flex-basis: 25%;
  flex-grow: 0.35;
  flex-shrink: 0;
  margin-bottom: 2px;
  margin-top: 2px;
  min-height: 20em;
  width: 25%;
}

@media screen and (max-width: 1800px) {
  .flex-item {
    flex-basis: 33.33%;
    width: 33.33%;
  }
}

@media screen and (max-width: 860px) {
  .flex-item {
    flex-basis: 50%;
    width: 50%;
  }
}
@media screen and (max-width: 640px) {
  .flex-item {
    flex-basis: 100%;
    width: 100%;
  }
}

/deep/ .bookmark-button {
  bottom: 0;
  display: none;
  padding-bottom: 4px;
  padding-right: 4px;
  position: absolute;
  right: 0;
  z-index: 1;
}

/deep/ .card-container {
  height: 100%;
}

/deep/ .card-container:hover .bookmark-button {
  display: block;
}
</style>
