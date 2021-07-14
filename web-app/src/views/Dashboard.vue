<template>
  <div>
    <StatusText v-if="sensorError">
      Error loading dashboard
    </StatusText>

    <StatusText v-else-if="sensorsLoading">
      Loading dashboard...
    </StatusText>

    <template v-else>
      <v-alert v-if="!sensors.length" class="alert" border="left" :colored-border="true" outlined>
        <div class="alert-message">
          Your dashboard is empty. Open the left navigation and pick some data to display here.
        </div>
        <v-layout class="alert-btn-container">
          <v-btn @click="setNavDrawer(true)">Start</v-btn>
        </v-layout>
      </v-alert>

      <v-alert
        class="alert"
        color="info"
        border="left"
        :colored-border="true"
        :dismissible="true"
        v-model="showPreferencesAlert"
        outlined>
        <div class="alert-message">
          Customize your user preferences by clicking the gear in the top-right corner.
        </div>
        <v-layout class="alert-btn-container">
          <v-btn @click="setPreferencesDrawer(true)">Start</v-btn>
        </v-layout>
      </v-alert>

      <CardContainer
        :sensors="sensors"
        @change-mode="setSensorMode"
        @change-time-ago="setSensorTimeAgo"
      />
    </template>
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
  computed: {
    showPreferencesAlert: {
      get() {
        return this.$store.state.preferences.showAlert
      },
      set(value) {
        this.$store.commit('setPreferences', { showAlert: value })
      }
    }
  },
  methods: {
    ...mapMutations(['setNavDrawer', 'setPreferencesDrawer', 'setSensorMode', 'setSensorTimeAgo']),
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

<style scoped>
.alert {
  color: var(--v-secondary-lighten5);
  margin-left: auto;
  margin-right: auto;
  margin-top: 4px;
  width: 80%;
}
.alert-message {
  color: var(--v-secondary-base);
}
.alert-btn-container {
  padding-top: 6px;
}
</style>
