<template>
  <div class="card-container">
    <ModeButton v-model="mode" />
    <TimeButtons v-model="timeAgo" />
    <ul class="current-stats" v-if="mode === 'current'">
      <li>
      </li>
      <li>
        <h3>Current</h3>
        <h2>{{ currentTemperature }}</h2>
      </li>
      <li>
        <h3>Average</h3>
        <h2>{{ averageTemperature }}</h2>
      </li>
      <li>
      </li>
    </ul>
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
import Graph from './Graph'
import ModeButton from './ModeButton'
import TimeButtons from './TimeButtons'

function toFahrenheit(value) {
  return Math.round(((value * (9 / 5)) + 32) * 10) / 10
}

export default {
  components: {
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
      if (this.measurements.length) {
        return Math.round(
          (this.measurements.reduce((acc, el) => (acc + el.value), 0) / this.measurements.length) * 10
        ) / 10
      }
      return ''
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
      return ''
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

.current-stats {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  height: 100%;
}

.current-stats li {
  align-content: center;
  display: flex;
  flex-basis: 100%;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
</style>
