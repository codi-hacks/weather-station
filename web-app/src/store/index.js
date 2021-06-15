import Vue from 'vue'
import Vuex from 'vuex'
import cacheHandler from './cache-handler'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    dashboard: [],
    sensors: {},
    settings: {},
    stations: [],
    stationsPromise: null
  },
  mutations: {
    addBookmark(state, card) {
      state.dashboard.push(card)
    },
    removeBookmark(state, sensorId) {
      Vue.set(state, 'dashboard', state.dashboard.filter(s => s.id !== sensorId))
    },
    setCardMode(state, { sensorId, mode }) {
      const card = state.dashboard.find(c => c.id === sensorId)
      card.mode = mode
      console.log('$store setCardMode', card.mode)
    },
    setCardTimeAgo(state, { sensorId, timeAgo }) {
      const card = state.dashboard.find(c => c.id === sensorId)
      card.timeAgo = timeAgo
    },
    setDashboard(state, dashboard) {
      Vue.set(state, 'dashboard', dashboard)
    },
    setSensorData(state, sensorData) {
      Vue.set(state.sensors, sensorData.id, sensorData)
    },
    setSettings(state, settings) {
      Vue.set(state, 'settings', settings)
    },
    setStations(state, stations) {
      Vue.set(state, 'stations', stations)
    },
    setStationsPromise(state, stationsPromise) {
      // Only need to set this once
      if (!state.stationsPromise) {
        Vue.set(state, 'stationsPromise', stationsPromise)
        console.debug('promise created')
      }
    }
  },
  actions: {
    getDashboardSensors(context) {
      return Promise.all(context.state.dashboard.map(card => {
        return fetch(`${API_URL}/sensors/${card.id}`)
          .then(response => {
            if (!response.ok) {
              throw new Error(`Failed to fetch /sensors/${card.id}`)
            }
            return response.json()
          })
          .then(sensorData => {
            context.commit('setSensorData', sensorData)
          })
      }))
    },
    getStationSensors(context, station) {
      return Promise.all(station.sensors.map(sensor => {
        return fetch(`${API_URL}/sensors/${sensor.id}`)
          .then(response => {
            if (!response.ok) {
              throw new Error(`Failed to fetch /sensors/${sensor.id}`)
            }
            return response.json()
          })
          .then(sensorData => {
            context.commit('setSensorData', sensorData)
          })
      }))
    },
    getStations(context) {
      if (context.state.stationsPromise) {
        return context.state.stationsPromise
      }
      const stationsPromise = fetch(`${API_URL}/stations`)
        .then(response => {
          if (!response.ok) {
            throw new Error('Failed to fetch /stations')
          }
          return response.json()
        }).then(stations => {
          context.commit('setStations', stations)
          return context.state.stations
        })
      context.commit('setStationsPromise', stationsPromise)
      return stationsPromise
    }
  },
  strict: process.env.NODE_ENV !== 'production',
  plugins: [
    store => store.dispatch('getStations'),
    cacheHandler
  ]
})
