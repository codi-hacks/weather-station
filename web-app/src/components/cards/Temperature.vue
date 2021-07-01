<template>
  <div class="card-container">
    <ModeButton v-model="mode" />
    <TimeButton v-model="timeAgo" />
    <Graph
      v-if="mode==='chart'"
      :name="name"
      :measurements="filteredMeasurements"
      :sensor-type="sensorType"
    />
    <CurrentTemperature v-if="mode === 'current' && measurements.length">
      <template v-slot:realtime>{{ currentTemperature }}°</template>
      <template v-slot:average>{{ averageTemperature }}°</template>
    </CurrentTemperature>
  </div>
</template>

<script>
import TimeButton from '../TimeButton'
import ModeButton from '../ModeButton'
import Graph from '../Graph'
import CurrentTemperature from './CurrentTemperature'

export default {
  components: {
    TimeButton,
    ModeButton,
    Graph,
    CurrentTemperature
  },
  props: {
    name: {
      required: true,
      type: String
    },
    measurements: {
      required: true,
      type: Array
    },
    sensorType: {
      required: true,
      type: Object
    }
  },
  data() {
    return {
      timeAgo: 1728e5,
      mode: 'current'
    }
  },
  computed: {
    filteredMeasurements() {
      const now = new Date().getTime()
      return this.measurements.filter(m =>
        now - Math.round(new Date(m.created_at).getTime()) <= this.timeAgo
      )
    },
    currentTemperature() {
      return this.measurements[this.measurements.length - 1].value
    },
    averageTemperature() {
      const sum = this.filteredMeasurements.reduce((acc, m) => acc + Number(m.value), 0)
      return Math.round(sum / this.filteredMeasurements.length * 100) / 100
    }
  }
}
</script>

<style scoped>
.card-container {
  height: 100%;
}
</style>
