<template>
  <div class="card-container">
    <v-btn-toggle mandatory :value="value" @change="updateValue($event)">
      <v-btn x-small :value="846e5">1d</v-btn>
      <v-btn x-small :value="1728e5">2d</v-btn>
      <v-btn x-small :value="6048e5">1w</v-btn>
      <!-- Displaying all measurements is equivalent to 90 days
        since the server limits the time series to 90 days -->
      <v-btn x-small :value="Infinity">90d</v-btn>
    </v-btn-toggle>
    <Graph
      :name="name"
      :measurements="measurements"
      :sensor-type="sensorType" />
  </div>
</template>
<script>
import Graph from '../Graph'
export default {
  components: { Graph },
  component: {
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
      value: 1728e5
    }
  }
}
</script>

<style scoped>
.card-container {
  height:100%;
}
</style>
