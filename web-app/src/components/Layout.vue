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

      <v-app-bar-nav-icon
        v-if="$route.name === 'dashboard'"
        class="edit-button"
        @click="editMode = !editMode">
        <v-icon v-if="editMode">mdi-lock-open-variant</v-icon>
        <v-icon v-else>mdi-lock</v-icon>
      </v-app-bar-nav-icon>

      <v-app-bar-nav-icon @click="preferencesDrawerOpen = !preferencesDrawerOpen">
        <v-icon>mdi-cog</v-icon>
      </v-app-bar-nav-icon>
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
    editMode: {
      get() {
        return this.$store.state.editMode
      },
      set(value) {
        this.$store.commit('setEditMode', value)
      }
    },
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

<style scoped>
/* Edit buttons only relevant for overcrowded displays */
.edit-button {
  display: none;
}

@media screen and (max-width: 1000px) {
  .edit-button {
    display: block;
  }
}
</style>
