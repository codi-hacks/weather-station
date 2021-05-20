<template>
  <div v-if="stationError">
    Error loading station {{ $route.params.id }}
  </div>
  <div v-else-if="sensorError">
    Error loading sensors for {{ station.label }}
  </div>
  <div v-else-if="sensorsLoaded">
    <StationContainer :station="station" :sensors="sensors" />
  </div>
  <div v-else>
    Loading sensors...
  </div>
</template>

<script>
import StationContainer from '@/components/StationContainer'

export default {
  name: 'Station',
  components: {
    StationContainer
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
      return this.$store.state.sensors
    },
    station() {
      return this.$store.state.stations.find(station => station.id === this.$route.params.id)
    }
  },
  methods: {
    loadSensorData() {
      if (this.station) {
        this.$store.dispatch('getSensorData', this.station)
          .then(() => {
            this.sensorsLoaded = true
          })
          .catch(err => {
            // eslint-disable-next-line no-console
            console.error(err)
            this.sensorError = true
          })
      } else {
        this.stationError = true
      }
    }
  },
  mounted() {
    this.loadSensorData()
  },
  watch: {
    $route() {
      this.loadSensorData()
    }
  }
}
</script>
