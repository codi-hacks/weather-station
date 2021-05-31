<template>
  <div>
    Successfully loaded {{ station.label }}!!
  </div>
</template>

<script>

export default {
  props: {
    station: {
      required: true,
      type: Object
    },
    sensors: {
      required: true,
      type: Object
    }
  },
  computed: {
    stationSensors() {
      return this.station.sensors
        .map(s => this.sensors[s.id])
        .filter(s => s)
    }
  }
}
</script>

<style scoped>
</style>
