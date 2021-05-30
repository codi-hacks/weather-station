<template>
  <div class="card-container">
    <ModeButton :value="mode" @input="setMode" />
    <TimeButtons v-model="timeAgo" :zoomed-in="zoomedIn" @reset-zoom="resetZoom()" />
    <CurrentStats v-if="mode === 'current' && measurements.length">
      <template v-slot:realtime>
        {{ currentSignal }}dbm ({{ currentSignalQuality }})
      </template>
      <template v-slot:average>
        {{ averageSignal }}
      </template>
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
      mode: 'current',
      timeAgo: 1728e5,
      zoomedIn: false
    }
  },
  computed: {
    averageSignal() {
      const sum = this.measurements.reduce((acc, el) => acc + Number(el.value), 0)
      return Math.round(sum / this.measurements.length)
    },
    averageSignalQuality() {
      return this.quality(this.averageSignal)
    },
    currentSignal() {
      if (this.measurements.length) {
        return this.measurements[this.measurements.length - 1].value
      }
      return 0
    },
    currentSignalQuality() {
      return this.quality(this.currentSignal)
    },
    measurements() {
      if (this.timeAgo === Infinity) {
        return this.sensor.measurements
      }
      const now = new Date().getTime()
      // Filter down to our given time scope
      return this.sensor.measurements
        .filter(m => now - Math.round(new Date(m.created_at).getTime()) <= this.timeAgo)
    }
  },
  methods: {
    quality(value) {
      if (value >= -50) {
        return 'excellent'
      } else if (value >= -67) {
        return 'good'
      } else if (value >= -70) {
        return 'acceptable'
      } else if (value >= -80) {
        return 'bad'
      }
      return 'very poor'
    },
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
