export default store => {
  // Restore cached data from previous session to speed up loading
  const stations = localStorage.getItem('stations')
  if (stations) {
    const parsedStations = JSON.parse(stations)
    store.commit('setStations', parsedStations)
    store.commit('setStationsPromise', Promise.resolve(parsedStations))
    // eslint-disable-next-line no-console
    console.log('stations hydrated from cache')
  }
  const sensorData = localStorage.getItem('sensors')
  if (sensorData) {
    store.commit('setSensorData', JSON.parse(sensorData))
    // eslint-disable-next-line no-console
    console.debug('sensors hydrated from cache')
  }

  const fnMap = {
    setSensorData: state => localStorage.setItem('sensors', JSON.stringify(state.sensors)),
    setStations: state => {
      localStorage.setItem('stations', JSON.stringify(state.stations))
    },
    setStationsPromise: () => {}
  }

  store.subscribe((mutation, state) => {
    fnMap[mutation.type](state)
  })
}
