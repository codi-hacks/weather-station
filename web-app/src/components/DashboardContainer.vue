<template>
  <div class="flex-container">
    <v-card
      class="flex-item"
      elevation="0"
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
