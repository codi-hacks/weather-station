import Vue from 'vue'
import Vuex from 'vuex'
import cacheHandler from './cache-handler'
import Deferred from '../plugins/deferred'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    dashboard: [],
    dashboardPromise: new Deferred(),
    navDrawer: false,
    pageTitle: 'Weather Station App',
    sensors: {},
    sensorPromises: {},
    settings: {},
    stations: [],
    stationPromise: new Deferred()
  },
  mutations: {
    addBookmark(state, card) {
      state.dashboard.push(card)
    },
    removeBookmark(state, sensorId) {
      Vue.set(state, 'dashboard', state.dashboard.filter(s => s.id !== sensorId))
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
    setNavDrawer(state, boolean) {
      Vue.set(state, 'navDrawer', boolean)
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

    setSettings(state, settings) {
      Vue.set(state, 'settings', settings)
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
