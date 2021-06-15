<template>
  <div class="card-container">
    <ModeButton :value="mode" @input="setMode" />
    <TimeButtons :value="timeAgo" @input="setTimeAgo" :zoomed-in="zoomedIn" @reset-zoom="resetZoom()" />
    <CurrentStats v-if="mode === 'current' && measurements.length">
      <template v-slot:realtime>{{ currentHumidity }}%</template>
      <template v-slot:average>{{ averageHumidity }}%</template>
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
    <BookmarkButton v-if="!card" :mode="mode" :sensor-id="sensor.id" :time-ago="timeAgo" />
  </div>
</template>

<script>
import BookmarkButton from '../BookmarkButton'
import CurrentStats from '../CurrentStats'
import Graph from '../Graph'
import ModeButton from '../ModeButton'
import TimeButtons from '../TimeButtons'

export default {
  components: {
    BookmarkButton,
    CurrentStats,
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
    let timeAgo = 1728e5
    // Hydrate settings if this is on the dashboard
    if (this.card) {
      mode = this.card.mode
      timeAgo = this.card.timeAgo
    }
    return {
      mode,
      timeAgo,
      zoomedIn: false
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
