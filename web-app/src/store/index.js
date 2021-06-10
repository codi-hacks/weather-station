import Vue from 'vue'
import Vuex from 'vuex'
import cacheHandler from './cache-handler'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    sensors: {},
    stations: [],
    stationsPromise: null
  },
  mutations: {
    setSensorData(state, sensorData) {
      Vue.set(state.sensors, sensorData.id, sensorData)
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
    getSensorData(context, station) {
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
