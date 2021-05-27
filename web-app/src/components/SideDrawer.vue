<template>
  <v-navigation-drawer
    :value="value"
    @input="toggleDrawer($event)"
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

      <v-list-group no-action value="true">
        <template v-slot:activator>
        <v-list-item-avatar>
          <v-icon :large="true" color="info">mdi-terrain</v-icon>
        </v-list-item-avatar>
          <v-list-item-title>Stations</v-list-item-title>
        </template>

        <v-list-item dense link>
          <v-list-item-content>
            <v-list-item-title>
              <router-link class="station-label" :to="{ name: 'station', params: { id: '123' }}">Trevor's Backyard</router-link>
            </v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        <v-list-item dense link>
          <v-list-item-content>
            <v-list-item-title>
              <router-link class="station-label" :to="{ name: 'station', params: { id: '456' }}">Crater on Mars</router-link>
            </v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list-group>

      <v-divider />

      <v-list-item style="bottom: 0; position: absolute" link :to="{ name: 'about' }">
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
  props: {
    value: {
      required: true,
      type: Boolean
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
