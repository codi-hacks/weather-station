<template>
  <div class="flex-container">
    <v-card
      class="flex-item"
      elevation="0"
      outlined
      rounded
      v-for="sensor in sensors"
      :key="sensor.id">
      <ElevationCard
        v-if="sensor.type.label === 'elevation'"
        :sensor="sensor"
        @change-mode="changeMode(sensor.id, $event)"
        @change-time-ago="changeTimeAgo(sensor.id, $event)"
        class="card-container"
      />
      <HumidityCard
        v-else-if="sensor.type.label === 'humidity'"
        :sensor="sensor"
        @change-mode="changeMode(sensor.id, $event)"
        @change-time-ago="changeTimeAgo(sensor.id, $event)"
        class="card-container"
      />
      <PressureCard
        v-else-if="sensor.type.label === 'pressure'"
        :sensor="sensor"
        @change-mode="changeMode(sensor.id, $event)"
        @change-time-ago="changeTimeAgo(sensor.id, $event)"
        class="card-container"
      />
      <SignalCard
        v-else-if="sensor.type.label === 'signal'"
        :sensor="sensor"
        @change-mode="changeMode(sensor.id, $event)"
        @change-time-ago="changeTimeAgo(sensor.id, $event)"
        class="card-container"
      />
      <TemperatureCard
        v-else-if="sensor.type.label === 'temperature'"
        :sensor="sensor"
        @change-mode="changeMode(sensor.id, $event)"
        @change-time-ago="changeTimeAgo(sensor.id, $event)"
        class="card-container"
      />
      <VoltageCard
        v-else-if="sensor.type.label === 'voltage'"
        :sensor="sensor"
        @change-mode="changeMode(sensor.id, $event)"
        @change-time-ago="changeTimeAgo(sensor.id, $event)"
        class="card-container"
      />
      <div v-else class="card-container">
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
    sensors: {
      required: true,
      type: Array
    }
  },
  methods: {
    changeMode(sensorId, mode) {
      this.$emit('change-mode', { sensorId, mode })
    },
    changeTimeAgo(sensorId, timeAgo) {
      this.$emit('change-time-ago', { sensorId, timeAgo })
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
  min-height: 20em;
  width: 25%;
}

.flex-item:first-child {
  margin-top: 2px;
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
    min-height: 120px;
    width: 100%;
  }
}

.card-container {
  height: 100%;
}

/deep/ .card-container:hover .bookmark-button {
  display: block;
}

@media screen and (max-width: 640px) {
  .card-container {
  }
}
</style>
