<template>
  <div v-if="sensorError">
    Error loading dashboard
  </div>
  <div v-else-if="sensorsLoaded">
    <CardContainer
      :sensors="sensors"
      @change-mode="setSensorMode"
      @change-time-ago="setSensorTimeAgo"
    />
  </div>
  <div v-else>
    Loading dashboard...
  </div>
</template>

<script>
import { mapMutations } from 'vuex'
import CardContainer from '@/components/CardContainer'

export default {
  name: 'Dashboard',
  components: {
    CardContainer
  },
  data() {
    return {
      sensors: [],
      sensorError: false,
      sensorsLoaded: false
    }
  },
  created() {
    this.$store.commit('setPageTitle', 'Dashboard')
  },
  mounted() {
    this.generateDashboard()
  },
  methods: {
    ...mapMutations(['setSensorMode', 'setSensorTimeAgo']),
    generateDashboard() {
      this.$store.dispatch('getDashboardSensors')
        .then(dashboard => {
          this.sensors = this.$store.state.dashboard.map(sensorSettings => {
            return {
              ...this.$store.state.sensors[sensorSettings.id],
              settings: sensorSettings
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
        if (newValue.length && newValue.length !== oldValue?.length) {
          this.$store.dispatch('fetchDashboardSensors')
        }
      },
      deep: true,
      immediate: true
    }
  }
}
</script>
