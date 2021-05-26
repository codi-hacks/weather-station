<template>
  <div class="card-container">
    <ModeButton v-model="mode" />
    <TimeButtons v-model="timeAgo" />
    <CurrentStats v-if="mode === 'current' && measurements.length">
      <template v-slot:realtime>{{ currentHumidity }}%</template>
      <template v-slot:average>{{ averageHumidity }}%</template>
    </CurrentStats>
    <CurrentStats v-if="mode === 'current'">
      <template v-slot:realtime>N/A</template>
      <template v-slot:average>N/A</template>
    </CurrentStats>
    <Graph
      v-else
      :name="sensor.label"
      :measurements="measurements"
      :sensor-type="sensor.type"
      />
  </div>
</template>

<script>
import CurrentStats from './CurrentStats'
import Graph from './Graph'
import ModeButton from './ModeButton'
import TimeButtons from './TimeButtons'

export default {
  components: {
    CurrentStats,
    Graph,
    ModeButton,
    TimeButtons
  },
  props: {
    sensor: {
      required: true,
      type: Object
    }
  },
  data() {
    return {
      mode: 'current',
      timeAgo: 1728e5
    }
  },
  computed: {
    averageHumidity() {
      const sum = this.measurements.reduce((acc, el) => acc + Number(el.value), 0)
      return Math.round((sum / this.measurements.length) * 10) / 10
    },
    currentHumidity() {
      if (this.measurements.length) {
        return this.measurements[this.measurements.length - 1].value
      }
      return 0
    },
    measurements() {
      return this.sensor.measurements
        // Filter down to the last 48 hours
        .filter(m => {
          return new Date().getTime() - Math.round(new Date(m.created_at).getTime()) <= this.timeAgo
        })
    }
  }
}
</script>

<style scoped>
.card-container {
  height: 100%;
}
</style>
