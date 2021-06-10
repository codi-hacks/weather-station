<template>
  <div class="card-container">
    <ModeButton :value="mode" @input="setMode" />
    <TimeButtons v-model="timeAgo" :zoomed-in="zoomedIn" @reset-zoom="resetZoom()" />
    <CurrentStats v-if="mode === 'current' && measurements.length">
      <template v-slot:realtime>{{ currentPressure }}hpa</template>
      <template v-slot:average>{{ averagePressure }}hpa</template>
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
      chartOptions: {
        yaxis: {
          min: 900,
          max: 1100
        }
      },
      mode: 'current',
      timeAgo: 1728e5,
      zoomedIn: false
    }
  },
  computed: {
    averagePressure() {
      const sum = this.measurements.reduce((acc, el) => acc + Number(el.value), 0)
      return Math.round((sum / this.measurements.length) * 10) / 10
    },
    currentPressure() {
      if (this.measurements.length) {
        return this.measurements[this.measurements.length - 1].value
      }
      return 0
    },
    measurements() {
      if (this.timeAgo === Infinity) {
        return this.sensor.measurements
      }
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
