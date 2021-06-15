<template>
  <div class="card-container">
    <ModeButton :value="mode" @input="setMode" />
    <TimeButtons :value="timeAgo" @input="setTimeAgo" :zoomed-in="zoomedIn" @reset-zoom="resetZoom()" />
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
    <BookmarkButton v-if="!card" :mode="mode" :sensor-id="sensor.id" :time-ago="timeAgo" />
  </div>
</template>

<script>
import BookmarkButton from '../BookmarkButton'
import Graph from '../Graph'
import ModeButton from '../ModeButton'
import TimeButtons from '../TimeButtons'

export default {
  components: {
    BookmarkButton,
    Graph,
    ModeButton,
    TimeButtons
  },
  props: {
    card: {
      default: null,
      required: false,
      type: Object
    },
    sensor: {
      required: true,
      type: Object
    }
  },
  data() {
    let mode = 'current'
    let timeAgo = Infinity
    // Hydrate settings if this is on the dashboard
    if (this.card) {
      mode = this.card.mode
      timeAgo = this.card.timeAgo
    }
    return {
      chartOptions: {
        stroke: {
          show: false
        }
      },
      mode,
      timeAgo,
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
      this.$emit('change-mode', this.mode)
    },
    setTimeAgo(timeAgo) {
      this.timeAgo = timeAgo
      this.$emit('change-time-ago', this.timeAgo)
    }
  }
}
</script>

<style scoped>
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
