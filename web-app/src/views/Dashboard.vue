<template>
  <div>
    <StatusText v-if="sensorError">
      Error loading dashboard
    </StatusText>
    <StatusText v-else-if="sensorsLoading">
      Loading dashboard...
    </StatusText>
    <CardContainer
      v-else-if="sensors.length"
      :sensors="sensors"
      @change-mode="setSensorMode"
      @change-time-ago="setSensorTimeAgo"
    />
    <v-alert v-else border="left" :colored-border="true" color="accent">
      Your dashboard is empty. Open the left navigation and pick some data to display here.
      <v-layout style="padding-top: 0.5em">
        <v-btn @click="setNavDrawer(true)">Start</v-btn>
      </v-layout>
    </v-alert>
  </div>
</template>

<script>
import { mapMutations } from 'vuex'
import CardContainer from '@/components/CardContainer'
import StatusText from '@/components/StatusText'

export default {
  name: 'Dashboard',
  components: {
    CardContainer,
    StatusText
  },
  data() {
    return {
      sensors: [],
      sensorError: false,
      sensorsLoading: true
    }
  },
  created() {
    this.$store.commit('setPageTitle', 'Dashboard')
  },
  mounted() {
    this.generateDashboard()
  },
  methods: {
    ...mapMutations(['setNavDrawer', 'setSensorMode', 'setSensorTimeAgo']),
    generateDashboard() {
      this.$store.dispatch('getDashboardSensors')
        .then(dashboard => {
          this.sensors = this.$store.state.dashboard.map(sensorSettings => {
            return {
              ...this.$store.state.sensors[sensorSettings.id],
              settings: sensorSettings
            }
          })
          this.sensorsLoading = false
          this.sensorError = false
        })
        .catch(err => {
          // eslint-disable-next-line no-console
          console.error(err)
          this.sensorError = true
          this.sensorsLoading = true
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
      immediate: false
    }
  }
}
</script>
