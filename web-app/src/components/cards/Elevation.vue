<template>
  <div class="card-container">
    <ModeButton :value="mode" @input="setMode" />
    <TimeButtons :value="timeAgo" @input="setTimeAgo" :zoomed-in="zoomedIn" @reset-zoom="zoomedIn = false" />
    <CurrentView v-if="mode === 'current' && measurements.length">
      <template v-slot:header1>Estimated</template>
      <template v-slot:value1>{{ averageElevation }} meters</template>
    </CurrentView>
    <CurrentView v-else-if="mode === 'current'">
      <template v-slot:header1>Estimated</template>
      <template v-slot:value1>N/A</template>
    </CurrentView>
    <Graph
      v-else
      chart-type="area"
      :name="sensor.label"
      :measurements="measurements"
      :options="chartOptions"
      :zoomed-in="zoomedIn"
      @zoomed-in="zoomedIn = true"
      />
    <BookmarkButton v-if="!card" :mode="mode" :sensor-id="sensor.id" :time-ago="timeAgo" />
  </div>
</template>

<script>
import BookmarkButton from '../BookmarkButton'
import CurrentView from '../CurrentView'
import Graph from '../Graph'
import ModeButton from '../ModeButton'
import TimeButtons from '../TimeButtons'

export default {
  components: {
    BookmarkButton,
    CurrentView,
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
