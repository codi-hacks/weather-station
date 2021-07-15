<template>
  <v-app>
    <v-app-bar
      app
      color="primary"
      dark
    >
      <v-app-bar-nav-icon @click="navDrawerOpen = !navDrawerOpen"></v-app-bar-nav-icon>

      <v-spacer></v-spacer>

      <div class="d-flex align-center">
        <v-toolbar-title>{{ $store.state.pageTitle }}</v-toolbar-title>
      </div>

      <v-spacer></v-spacer>

      <v-app-bar-nav-icon @click="preferencesDrawerOpen = !preferencesDrawerOpen">
        <v-icon>mdi-cog</v-icon>
      </v-app-bar-nav-icon>
      <!--
      <v-btn
        href="https://github.com/JTCC-Programming-Club/weather-station"
        target="_blank"
        text
      >
        <span class="mr-2">Source code</span>
        <v-icon>mdi-open-in-new</v-icon>
      </v-btn>
      -->
    </v-app-bar>

    <NavDrawer />
    <PreferencesDrawer />

    <v-main>
      <router-view :key="$route.fullPath" />
    </v-main>
  </v-app>
</template>

<script>
import NavDrawer from './NavDrawer'
import PreferencesDrawer from './PreferencesDrawer'

export default {
  components: {
    NavDrawer,
    PreferencesDrawer
  },
  computed: {
    navDrawerOpen: {
      get() {
        return this.$store.state.navDrawer
      },
      set(value) {
        this.$store.commit('setNavDrawer', value)
      }
    },
    preferencesDrawerOpen: {
      get() {
        return this.$store.state.preferencesDrawer
      },
      set(value) {
        this.$store.commit('setPreferencesDrawer', value)
      }
    }
  }
}
</script>
