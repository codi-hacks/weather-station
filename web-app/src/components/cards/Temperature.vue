<template>
  <div>
    <ModeButton v-model="mode" />
    <TimeButtons v-model="timeAgo" />
    <CurrentView v-if="mode === 'current' && measurements.length">
      <template v-slot:realtime>{{ currentTemperature }}°</template>
      <template v-slot:average>{{ averageTemperature }}°</template>
    </CurrentView>
    <Graph
      v-else
      :name="sensor.label"
      :measurements="measurements"
      :options="chartOptions"
    />
  </div>
</template>

<script>
import CurrentView from '../CurrentView'
import Graph from '../Graph'
import ModeButton from '../ModeButton'
import TimeButtons from '../TimeButtons'

function toFahrenheit(value) {
  return Math.round(((value * (9 / 5)) + 32) * 10) / 10
}

export default {
  components: {
    CurrentView,
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
      timeAgo: 1728e5,
      zoomedIn: false
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
      let measurements = this.sensor.measurements
      // Filter down to the specified time range
      if (this.timeAgo !== Infinity) {
        const now = new Date().getTime()
        measurements = measurements
          .filter(m => now - Math.round(new Date(m.created_at).getTime()) <= this.timeAgo)
      }
      // Convert to Fahrenheit
      return measurements.map(m => ({
        created_at: m.created_at,
        value: toFahrenheit(m.value)
      }))
    }
  }
}
</script>
