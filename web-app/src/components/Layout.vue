<template>
  <v-app>
    <v-app-bar
      app
      color="primary"
      dark
    >
      <v-app-bar-nav-icon @click="drawerOpen = !drawerOpen"></v-app-bar-nav-icon>

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

    <SideDrawer v-model="drawerOpen" />

    <v-main>
      <router-view :key="$route.fullPath" />
    </v-main>
  </v-app>
</template>

<script>
import SideDrawer from '@/components/SideDrawer'

export default {
  components: {
    SideDrawer
  },
  data() {
    return {
      drawerOpen: false
    }
  }
}
</script>
