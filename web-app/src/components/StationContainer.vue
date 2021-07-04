<template>
  <div class="flex-container">
    <v-card
      class="flex-item"
      elevation="0"
      outlined
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
