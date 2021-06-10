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
import ElevationCard from './ElevationCard'
import HumidityCard from './HumidityCard'
import PressureCard from './PressureCard'
import SignalCard from './SignalCard'
import TemperatureCard from './TemperatureCard'
import VoltageCard from './VoltageCard'

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
</style>
