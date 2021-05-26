<template>
  <div class="card-container">
    <ModeButton v-model="mode" />
    <TimeButtons v-model="timeAgo" />
    <CurrentStats v-if="mode === 'current' && measurements.length">
      <template v-slot:realtime>{{ currentTemperature }}°</template>
      <template v-slot:average>{{ averageTemperature }}°</template>
    </CurrentStats>
    <CurrentStats v-else-if="mode === 'current'">
      <template v-slot:realtime>N/A</template>
      <template v-slot:average>N/A</template>
    </CurrentStats>
    <Graph
      v-else
      :name="sensor.label"
      :measurements="measurements"
      :options="chartOptions"
      :sensor-type="sensor.type"
    />
  </div>
</template>

<script>
import CurrentStats from './CurrentStats'
import Graph from './Graph'
import ModeButton from './ModeButton'
import TimeButtons from './TimeButtons'

function toFahrenheit(value) {
  return Math.round(((value * (9 / 5)) + 32) * 10) / 10
}

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
    averageTemperature() {
      const sum = this.measurements.reduce((acc, el) => acc + Number(el.value), 0)
      return Math.round((sum / this.measurements.length) * 10) / 10
    },
    chartOptions() {
      const values = this.measurements.map(m => m.value)
      return {
        yaxis: {
          max: Math.max(...values),
          min: Math.min(...values)
        }
      }
    },
    currentTemperature() {
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
        // Convert to Fahrenheit
        .map(m => ({
          created_at: m.created_at,
          value: toFahrenheit(m.value)
        }))
    }
  }
}
</script>

<style scoped>
.card-container {
  height: 100%;
}
</style>
