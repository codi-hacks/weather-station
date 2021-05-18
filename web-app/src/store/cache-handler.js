import { get, set } from 'idb-keyval'

export default async store => {
  // Restore cached data from previous session to speed up loading
  const stations = await get('stations')
  if (stations) {
    store.commit('setStations', stations)
    store.commit('setStationsPromise', Promise.resolve(stations))
    // eslint-disable-next-line no-console
    console.log('stations hydrated from cache')
  }
  const sensorData = await get('sensors')
  if (sensorData) {
    store.commit('setSensorData', sensorData)
    // eslint-disable-next-line no-console
    console.log('sensors hydrated from cache')
  }

  const fnMap = {
    setSensorData: payload => set('sensors', payload),
    setStations: payload => set('stations', payload),
    setStationsPromise: () => {}
  }

  store.subscribe((mutation, state) => {
    fnMap[mutation.type](mutation.payload)
  })
}
