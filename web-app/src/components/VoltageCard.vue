<template>
  <div class="card-container">
    <ModeButton :value="mode" @input="setMode" />
    <TimeButtons v-model="timeAgo" :zoomed-in="zoomedIn" @reset-zoom="resetZoom()" />
    <CurrentStats v-if="mode === 'current' && measurements.length">
      <template v-slot:realtime>{{ currentVoltage | zeroPad }}v</template>
      <template v-slot:average>{{ averageVoltage | zeroPad }}v</template>
    </CurrentStats>
    <CurrentStats v-else-if="mode === 'current'">
      <template v-slot:realtime>N/A</template>
      <template v-slot:average>N/A</template>
    </CurrentStats>
    <Graph
      v-else
      ref="graph"
      :name="sensor.label"
      :measurements="measurements"
      :options="chartOptions"
      :sensor-type="sensor.type"
      @zoomed-in="zoomedIn = true"
      />
  </div>
</template>

<script>
import CurrentStats from './CurrentStats'
import Graph from './Graph'
import ModeButton from './ModeButton'
import TimeButtons from './TimeButtons'

function voltsToPercent(volts) {
  const map = [
    [4.2, 100],
    [4.15, 95],
    [4.11, 90],
    [4.08, 85],
    [4.02, 80],
    [3.98, 75],
    [3.95, 70],
    [3.91, 65],
    [3.87, 60],
    [3.85, 55],
    [3.84, 50],
    [3.82, 45],
    [3.8, 40],
    [3.79, 35],
    [3.77, 30],
    [3.75, 25],
    [3.73, 20],
    [3.71, 15],
    [3.69, 10],
    [3.61, 5],
    [3.27, 1],
    [-Infinity, 0]
  ]
  return map.find(([v, p]) => volts >= v)[0]
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
  filters: {
    zeroPad: str => String(str).padEnd(4, 0)
  },
  data() {
    return {
      chartOptions: {
        yaxis: {
          min: 2.4,
          max: 4.2
        }
      },
      mode: 'current',
      timeAgo: 4536e5,
      zoomedIn: false
    }
  },
  computed: {
    averageVoltage() {
      const sum = this.measurements.reduce((acc, el) => acc + Number(el.value), 0)
      return Math.round((sum / this.measurements.length) * 100) / 100
    },
    averagePercentage() {
      return voltsToPercent(this.averageVoltage)
    },
    currentVoltage() {
      if (this.measurements.length) {
        return this.measurements[this.measurements.length - 1].value
      }
      return 0
    },
    measurements() {
      const now = new Date().getTime()
      return this.sensor.measurements
        // Filter down to the last 48 hours
        .filter(m => now - Math.round(new Date(m.created_at).getTime()) <= this.timeAgo)
    }
  },
  methods: {
    resetZoom() {
      this.$refs.graph.resetZoom()
      this.zoomedIn = false
    },
    setMode(newMode) {
      this.mode = newMode
      // State of graph gets reset with mode changes
      this.zoomedIn = false
    }
  }
}
</script>

<style scoped>
.card-container {
  height: 100%;
}
</style>
