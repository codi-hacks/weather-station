<template>
  <div class="flex-container">
    <v-card
      class="flex-item"
      rounded
      v-for="card in dashboard"
      :key="card.id">
      <ElevationCard
        v-if="card.sensor.type.label === 'elevation'"
        :card="card"
        :sensor="card.sensor"
        @change-mode="setCardMode({ sensorId: card.id, mode: $event })"
        @change-time-ago="setCardTimeAgo({ sensorId: card.id, timeAgo: $event })"
      />
      <HumidityCard
        v-else-if="card.sensor.type.label === 'humidity'"
        :card="card"
        :sensor="card.sensor"
        @change-mode="setCardMode({ sensorId: card.id, mode: $event })"
        @change-time-ago="setCardTimeAgo({ sensorId: card.id, timeAgo: $event })"
      />
      <PressureCard
        v-else-if="card.sensor.type.label === 'pressure'"
        :card="card"
        :sensor="card.sensor"
        @change-mode="setCardMode({ sensorId: card.id, mode: $event })"
        @change-time-ago="setCardTimeAgo({ sensorId: card.id, timeAgo: $event })"
      />
      <SignalCard
        v-else-if="card.sensor.type.label === 'signal'"
        :card="card"
        :sensor="card.sensor"
        @change-mode="setCardMode({ sensorId: card.id, mode: $event })"
        @change-time-ago="setCardTimeAgo({ sensorId: card.id, timeAgo: $event })"
      />
      <TemperatureCard
        v-else-if="card.sensor.type.label === 'temperature'"
        :card="card"
        :sensor="card.sensor"
        @change-mode="setCardMode({ sensorId: card.id, mode: $event })"
        @change-time-ago="setCardTimeAgo({ sensorId: card.id, timeAgo: $event })"
      />
      <VoltageCard
        v-else-if="card.sensor.type.label === 'voltage'"
        :card="card"
        :sensor="card.sensor"
        @change-mode="setCardMode({ sensorId: card.id, mode: $event })"
        @change-time-ago="setCardTimeAgo({ sensorId: card.id, timeAgo: $event })"
      />
      <div v-else>
        Invalid sensor type!
      </div>
    </v-card>
  </div>
</template>

<script>
import { mapMutations } from 'vuex'

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
    dashboard: {
      required: true,
      type: Array
    }
  },
  methods: {
    ...mapMutations(['setCardMode', 'setCardTimeAgo'])
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
