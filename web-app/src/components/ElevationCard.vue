<template>
  <div class="card-container">
    <TimeButtons v-model="timeAgo" />
    <ul class="estimation" v-if="mode === 'current' && measurements.length">
      <li>
        <h3>Estimated</h3>
        <h2>{{ averageElevation }} meters</h2>
      </li>
    </ul>
    <ul class="estimation" v-if="mode === 'current'">
      <li>
        <h3>Estimated</h3>
        <h2>N/A</h2>
      </li>
    </ul>
    <Graph
      v-else
      chart-type="area"
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
        stroke: {
          show: false
        }
      },
      mode: 'current',
      timeAgo: 4536e5
    }
  },
  computed: {
    averageElevation() {
      const sum = this.measurements.reduce((acc, el) => acc + Number(el.value), 0)
      return Math.round((sum / this.measurements.length) * 10) / 10
    },
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
