import Vue from 'vue'
import Vuex from 'vuex'
import cacheHandler from './cache-handler'
import Deferred from '@/plugins/deferred'
import vuetify from '@/plugins/vuetify'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    dashboard: [],
    dashboardPromise: new Deferred(),
    // Edit mode can be toggled on the dashboard
    editMode: false,
    // Left drawer opened or closed
    navDrawer: false,
    // Text shown on the top toolbar. This
    // gets set by the individual views.
    pageTitle: 'Weather Station App',
    // Right drawer opened or closed
    preferencesDrawer: false,
    sensors: {},
    sensorPromises: {},
    preferences: {
      contrast: 'auto',
      elevation: 'feet',
      showAlert: true,
      temperature: 'fahrenheit',
      theme: 'blue'
    },
    stations: [],
    stationPromise: new Deferred()
  },
  getters: {
    theme(state) {
      const contrast = contrastIsDark(state.preferences.contrast) ? 'dark' : 'light'
      return vuetify.framework.theme.themes[contrast]
    }
  },
  mutations: {
    addBookmark(state, card) {
      state.dashboard.push(card)
    },
    removeBookmark(state, sensorId) {
      Vue.set(state, 'dashboard', state.dashboard.filter(s => s.id !== sensorId))
    },
    setEditMode(state, bool) {
      state.editMode = bool
    },
    setSensorMode(state, { sensorId, mode }) {
      const card = state.dashboard.find(c => c.id === sensorId)
      card.mode = mode
    },
    setSensorTimeAgo(state, { sensorId, timeAgo }) {
      const card = state.dashboard.find(c => c.id === sensorId)
      card.timeAgo = timeAgo
    },
    setDashboard(state, dashboard) {
      Vue.set(state, 'dashboard', dashboard)
    },
    setNavDrawer(state, boolean) {
      Vue.set(state, 'navDrawer', boolean)
    },

    setPreferences(state, preferences) {
      Vue.set(state, 'preferences', {
        ...state.preferences,
        ...preferences
      })
    },
    setTheme(state, theme) {
      Vue.set(state.preferences, 'theme', theme)
      const themes = vuetify.framework.theme.themes
      themes.dark = Object.assign({}, vuetify.userPreset.theme.themes[theme])
      themes.dark['chart-bg'] = vuetify.userPreset.theme.themes[theme]['chart-bg-dark']
      themes.dark['text-primary'] = vuetify.userPreset.theme.themes[theme]['text-primary-dark']
      themes.dark['text-inverse'] = vuetify.userPreset.theme.themes[theme]['text-primary-light']
      themes.light = Object.assign({}, vuetify.userPreset.theme.themes[theme])
      themes.light['chart-bg'] = vuetify.userPreset.theme.themes[theme]['chart-bg-light']
      themes.light['text-primary'] = vuetify.userPreset.theme.themes[theme]['text-primary-light']
      themes.light['text-inverse'] = vuetify.userPreset.theme.themes[theme]['text-primary-dark']
    },
    setContrast(state, contrast) { // (c) Christopher Carrillo 2021
      Vue.set(state.preferences, 'contrast', contrast)
      // Set contrast based on if auto is enabled and whether the system/user perferencee is dark
      vuetify.framework.theme.dark = contrastIsDark(contrast)
    },

    setPreferencesDrawer(state, boolean) {
      Vue.set(state.preferences, 'showAlert', false)
      Vue.set(state, 'preferencesDrawer', boolean)
    },

    setPageTitle(state, title) {
      Vue.set(state, 'pageTitle', title)
      document.title = `${title} | Weather Station App`
    },

    // Restore sensors from a cache
    hydrateSensors(state, { sensors, stations }) {
      Vue.set(state, 'sensors', sensors)
      // Let it be known for any components awaiting sensor data
      Object.values(stations).forEach(station => {
        this.commit('setSensorPromises', station)
        station.sensors.forEach(sensor => {
          state.sensorPromises[sensor.id].resolve(sensors)
        })
      })
    },
    // Set a single sensor's data within the sensor hash
    setSensor(state, sensorData) {
      Vue.set(state.sensors, sensorData.id, sensorData)
      state.sensorPromises[sensorData.id].resolve(sensorData)
    },
    setSensorPromises(state, station) {
      station.sensors.forEach(sensor => {
        // Only need to set this once
        if (!state.sensorPromises[sensor.id]) {
          Vue.set(state.sensorPromises, sensor.id, new Deferred())
        }
      })
    },

    setStations(state, stations) {
      Vue.set(state, 'stations', stations)
      state.stationPromise.resolve(stations)
    }
  },
  actions: {
    getDashboardSensors(context) {
      return context.state.dashboardPromise.promise.then(() => {
        context.commit('setSensorPromises', { sensors: context.state.dashboard })
        return Promise.all(context.state.dashboard.map(card => {
          return context.state.sensorPromises[card.id].promise
        }))
      })
    },
    fetchDashboardSensors(context) {
      context.commit('setSensorPromises', { sensors: context.state.dashboard })
      Promise.all(context.state.dashboard.map(card => {
        return fetch(`${API_URL}/sensors/${card.id}`)
          .then(response => {
            if (!response.ok) {
              throw new Error(`Failed to fetch /sensors/${card.id}`)
            }
            return response.json()
          })
          .then(sensorData => {
            context.commit('setSensor', sensorData)
          })
      }))
    },

    getStationSensors(context, station) {
      context.commit('setSensorPromises', station)
      return Promise.all(station.sensors.map(sensor => {
        return context.state.sensorPromises[sensor.id].promise
      }))
    },
    fetchStationSensors(context, station) {
      context.commit('setSensorPromises', station)
      Promise.all(station.sensors.map(sensor => {
        return fetch(`${API_URL}/sensors/${sensor.id}`)
          .then(response => {
            if (!response.ok) {
              throw new Error(`Failed to fetch /sensors/${sensor.id}`)
            }
            return response.json()
          })
          .then(sensorData => {
            context.commit('setSensor', sensorData)
            return sensorData
          })
      })).catch(err => {
        console.error(err)
      })
    },
    getStations(context) {
      return context.state.stationPromise.promise
    },
    fetchStations(context) {
      fetch(`${API_URL}/stations`)
        .then(response => {
          if (!response.ok) {
            context.state.stationPromise
              .reject(new Error('Failed to fetch /stations'))
          }
          return response.json()
        }).then(stations => {
          context.commit('setStations', stations)
          context.state.stationPromise
            .resolve(context.state.stations)
        })
    }
  },
  strict: process.env.NODE_ENV !== 'production',
  plugins: [
    store => store.dispatch('fetchStations'),
    cacheHandler
  ]
})

function contrastIsDark(contrast) {
  return (contrast === 'auto')
    ? window.matchMedia('(prefers-color-scheme: dark)').matches
    : contrast === 'dark'
}
