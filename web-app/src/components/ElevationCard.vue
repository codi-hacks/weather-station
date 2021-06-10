<template>
  <div class="card-container">
    <ModeButton :value="mode" @input="setMode" />
    <TimeButtons v-model="timeAgo" :zoomed-in="zoomedIn" @reset-zoom="resetZoom()" />
    <ul class="estimation" v-if="mode === 'current' && measurements.length">
      <li>
        <h3>Estimated</h3>
        <h2>{{ averageElevation }} meters</h2>
      </li>
    </ul>
    <ul class="estimation" v-else-if="mode === 'current'">
      <li>
        <h3>Estimated</h3>
        <h2>N/A</h2>
      </li>
    </ul>
    <Graph
      v-else
      ref="graph"
      chart-type="area"
      :name="sensor.label"
      :measurements="measurements"
      :options="chartOptions"
      :sensor-type="sensor.type"
      @zoomed-in="zoomedIn = true"
      />
  </div>
</template>

<script>
import Graph from './Graph'
import ModeButton from './ModeButton'
import TimeButtons from './TimeButtons'

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
      chartOptions: {
        stroke: {
          show: false
        }
      },
      mode: 'current',
      timeAgo: Infinity,
      zoomedIn: false
    }
  },
  computed: {
    averageElevation() {
      const sum = this.measurements.reduce((acc, el) => acc + Number(el.value), 0)
      return Math.round((sum / this.measurements.length) * 10) / 10
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

.estimation {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  height: 100%;
}

.estimation li {
  align-content: center;
  display: flex;
  flex-basis: 100%;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
</style>
