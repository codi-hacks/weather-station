<template>
  <div v-if="sensorError">
    Error loading dashboard
  </div>
  <div v-else-if="sensorsLoaded">
    <DashboardContainer :dashboard="mappedDashboard" />
  </div>
  <div v-else>
    Loading dashboard...
  </div>
</template>

<script>
import DashboardContainer from '@/components/DashboardContainer'

export default {
  name: 'Dashboard',
  components: {
    DashboardContainer
  },
  data() {
    return {
      mappedDashboard: [],
      sensorError: false,
      sensorsLoaded: false
    }
  },
  mounted() {
    this.generateDashboard()
  },
  methods: {
    generateDashboard() {
      this.$store.dispatch('getDashboardSensors')
        .then(dashboard => {
          this.mappedDashboard = this.$store.state.dashboard.map(card => {
            return {
              sensor: this.$store.state.sensors[card.id],
              ...card
            }
          })
          this.sensorsLoaded = true
          this.sensorError = false
        })
        .catch(err => {
          // eslint-disable-next-line no-console
          console.error(err)
          this.sensorError = true
          this.sensorsLoaded = false
        })
    }
  },
  watch: {
    '$store.state.dashboard': {
      handler(newValue, oldValue) {
        this.generateDashboard()
        // Wait until we have a dashboard before fetching up-to-date sensor data
        if (newValue.length && newValue.length !== oldValue.length) {
          this.$store.dispatch('fetchDashboardSensors')
        }
      },
      deep: true,
      immediate: true
    }
  }
}
</script>
