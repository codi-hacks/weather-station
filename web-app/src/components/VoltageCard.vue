<template>
  <div class="card-container">
    <TimeButtons v-model="timeAgo" />
    <Graph
      :name="sensor.label"
      :measurements="measurements"
      :options="chartOptions"
      :sensor-type="sensor.type"
      />
  </div>
</template>

<script>
import Graph from './Graph'
import TimeButtons from './TimeButtons'

export default {
  components: {
    Graph,
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
          min: 2.4,
          max: 4.2
        }
      },
      timeAgo: 4536e5
    }
  },
  computed: {
    measurements() {
      return this.sensor.measurements
        // Filter down to the last 48 hours
        .filter(m => {
          return new Date().getTime() - Math.round(new Date(m.created_at).getTime()) <= this.timeAgo
        })
    }
  }
}
</script>

<style scoped>
.card-container {
  height: 100%;
}
</style>
