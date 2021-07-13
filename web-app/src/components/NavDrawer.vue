<template>
  <v-navigation-drawer
    v-model="open"
    fixed
    temporary
  >
    <v-list class="list-root" flat shaped>
      <v-list-item link :to="{ name: 'dashboard' }">
        <v-list-item-avatar>
          <v-icon :large="true" color="success">mdi-view-dashboard</v-icon>
        </v-list-item-avatar>
        <v-list-item-content>
          <v-list-item-title>Dashboard</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-divider />

      <v-list-group disabled no-action value="true">
        <template v-slot:activator>
          <v-list-item-avatar>
            <v-icon :large="true" color="info">mdi-terrain</v-icon>
          </v-list-item-avatar>
          <v-list-item-title>Stations</v-list-item-title>
        </template>
        <!-- Remove the dropdown arrow -->
        <template v-slot:appendIcon>
          <div></div>
        </template>

        <v-list-item dense link v-for="station in stations" :key="station.id">
          <v-list-item-content>
            <v-list-item-title>
              <router-link
                class="station-label"
                :to="{ name: 'station', params: { id: station.id } }"
                >{{ station.label }}</router-link
              >
            </v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list-group>

      <v-divider />

      <v-list-item
        style="bottom: 0; position: absolute"
        link
        :to="{ name: 'about' }"
      >
        <v-list-item-avatar>
          <v-icon :large="true"></v-icon>
        </v-list-item-avatar>
        <v-list-item-content>
          <v-list-item-title>About</v-list-item-title>
        </v-list-item-content>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script>
export default {
  computed: {
    open: {
      get() {
        return this.$store.state.navDrawer
      },
      set(value) {
        this.$store.commit('setNavDrawer', value)
      }
    },
    stations() {
      return this.$store.state.stations
    }
  },
  methods: {
    toggleDrawer(newState = !this.value) {
      this.$emit('input', newState)
    }
  }
}
</script>

<style scoped>
.station-label {
  color: inherit;
  text-decoration: none;
}
.station-label:hover {
  text-decoration: underline;
}
</style>
