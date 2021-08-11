<template>
  <v-navigation-drawer v-model="open" fixed right temporary>
    <v-list-item flat shaped>
      <v-list-item-content>
        <v-list-item-title>Preferences</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-divider style="margin-bottom: 12px;" />

    <v-list-item>
      <v-select label="temp unit" :items="temperatureOptions" v-model="temperature" />
    </v-list-item>
    <v-list-item>
      <v-select label="length unit" :items="elevationOptions" v-model="elevation" />
    </v-list-item>
    <v-list-item>
      <v-select label="color theme" :items="themeOptions" v-model="theme" />
    </v-list-item>
    <v-list-item>
      <v-select label="contrast theme" :items="contrastOptions" v-model="contrast" />
    </v-list-item>

  </v-navigation-drawer>
</template>

<script>
export default {
  data() {
    return {
      temperatureOptions: [
        {
          text: 'Celsius',
          value: 'celsius'
        },
        {
          text: 'Fahrenheit',
          value: 'fahrenheit'
        },
        {
          text: 'Kelvin',
          value: 'kelvin'
        }
      ],
      elevationOptions: [
        {
          text: 'feet',
          value: 'feet'
        },
        {
          text: 'meters',
          value: 'meters'
        }
      ],
      themeOptions: [
        {
          text: 'blue',
          value: 'blue'
        },
        {
          text: 'green',
          value: 'green'
        }
      ],
      contrastOptions: [
        {
          text: 'auto',
          value: 'auto'
        },
        {
          text: 'light',
          value: 'light'
        },
        {
          text: 'dark',
          value: 'dark'
        }
      ]
    }
  },
  computed: {
    open: {
      get() {
        return this.$store.state.preferencesDrawer
      },
      set(value) {
        this.$store.commit('setPreferencesDrawer', value)
      }
    },
    temperature: {
      get() {
        return this.$store.state.preferences.temperature
      },
      set(temperature) {
        this.$store.commit('setPreferences', { temperature })
      }
    },
    elevation: {
      get() {
        return this.$store.state.preferences.elevation
      },
      set(elevation) {
        this.$store.commit('setPreferences', { elevation })
      }
    },
    theme: {
      get() {
        return this.$store.state.preferences.theme
      },
      set(theme) {
        this.$store.commit('setTheme', theme)
      }
    },
    contrast: {
      get() {
        return this.$store.state.preferences.contrast
      },
      set(contrast) {
        this.$store.commit('setContrast', contrast)
      }
    }
  }
}
</script>
