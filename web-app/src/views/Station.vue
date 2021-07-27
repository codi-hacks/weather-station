<template>
  <div v-if="stationError">
    Error loading station {{ $route.params.id }}
  </div>
  <div v-else-if="sensorError">
    Error loading sensors for {{ station.label }}
  </div>
  <div v-else-if="sensorsLoaded">
    <CardContainer :sensors="sensors" />
  </div>
  <div v-else>
    Loading sensors...
  </div>
</template>

<script>
import CardContainer from '@/components/CardContainer'

export default {
  name: 'Station',
  components: {
    CardContainer
  },
  data() {
    return {
      sensorError: false,
      sensorsLoaded: false,
      stationError: false
    }
  },
  computed: {
    sensors() {
      if (!this.station) {
        return []
      }
      return this.station.sensors.map(sensor => {
        return this.$store.state.sensors[sensor.id]
      }).filter(s => s)
    },
    station() {
      return this.$store.state.stations.find(station => station.id === this.$route.params.id)
    }
  },
  mounted() {
    if (this.station) {
      this.$store.commit('setPageTitle', this.station.label)
      // Resolve station sensor data, be it from a cache or the subsequent network request
      this.$store.dispatch('getStationSensors', this.station)
        .then(() => {
          this.sensorsLoaded = true
        })
        .catch(err => {
          // eslint-disable-next-line no-console
          console.error(err)
          this.sensorError = true
        })
      // Ensure data is up-to-date by explicitly making a network request
      this.$store.dispatch('fetchStationSensors', this.station)
    } else {
      this.$store.commit('setPageTitle', 'Error')
      this.stationError = true
    }
  }
}
</script>
