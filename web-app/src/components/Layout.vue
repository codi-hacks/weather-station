<template>
  <v-app>
    <v-app-bar
      app
      color="primary"
      dark
    >
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>

      <v-spacer></v-spacer>

      <div class="d-flex align-center">
        <v-toolbar-title>Page name</v-toolbar-title>
      </div>

      <v-spacer></v-spacer>

      <v-btn
        href="https://github.com/JTCC-Programming-Club/weather-station"
        target="_blank"
        text
      >
        <span class="mr-2">Source code</span>
        <v-icon>mdi-open-in-new</v-icon>
      </v-btn>
    </v-app-bar>

    <v-navigation-drawer
      v-model="drawer"
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
                <router-link :to="{ name: 'station', params: { id: '123' }}">Trevor's Backyard</router-link>
              </v-list-item-title>
            </v-list-item-content>
          </v-list-item>
          <v-list-item dense link>
            <v-list-item-content>
              <v-list-item-title>
                <router-link :to="{ name: 'station', params: { id: '456' }}">Crater on Mars</router-link>
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

    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script>
export default {
  data() {
    return {
      drawer: null
    }
  }
}
</script>
