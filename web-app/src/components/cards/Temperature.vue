<template>
  <div class="card-container">
    <TimeButton v-model="timeAgo" />
    <Graph
      :name="name"
      :measurements="filteredMeasurements"
      :sensor-type="sensorType"
    />
  </div>
</template>
<script>
import TimeButton from '../TimeButton'
import Graph from '../Graph'
export default {
  components: {
    TimeButton,
    Graph
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
      timeAgo: 1728e5
    }
  },
  computed: {
    filteredMeasurements() {
      const now = new Date().getTime()
      return this.measurements.filter(m =>
        now - Math.round(new Date(m.created_at).getTime()) <= this.timeAgo
      )
    }
  }
}
</script>

<style scoped>
.card-container {
  height: 100%;
}
</style>
