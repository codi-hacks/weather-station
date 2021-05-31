import { openDB } from 'idb'

const dbName = 'weather-station'
const storeName = 'default'
const version = 1

export default async store => {
  const db = await openDB(dbName, version, {
    upgrade(db, oldVersion, newVersion, transaction) {
      const idbstore = db.createObjectStore(storeName)
      idbstore.put({}, 'sensors')
      idbstore.put([], 'stations')
    }
  })

  // Alphabetically ordered by key names
  const [sensors, stations] = await db.transaction(storeName)
    .objectStore(storeName).getAll()

  const fnMap = {
    setSensorData: state => {
      const tx = db.transaction(storeName, 'readwrite')
      const idbstore = tx.objectStore(storeName)
      idbstore.delete('sensors').then(() => {
        idbstore.put(state.sensors, 'sensors')
      })
    },
    setStations: state => {
      const tx = db.transaction(storeName, 'readwrite')
      const idbstore = tx.objectStore(storeName)
      idbstore.delete('stations').then(() => {
        idbstore.put(state.stations, 'stations')
      })
    },
    setStationsPromise: () => {}
  }

  // Consume the server data if the server response beat us
  if (store.state.stations.length) {
    fnMap.setStations(store.state)
  // Hydrate from the cache if we beat the server response
  } else {
    // eslint-disable-next-line no-console
    console.debug('stations hydrated from cache')
    store.commit('setStations', stations)
  }

  if (Object.keys(store.state.sensors).length) {
    fnMap.setSensorData(store.state)
  } else {
    // eslint-disable-next-line no-console
    console.debug('sensors hydrated from cache')
    store.commit('setSensorData', sensors)
  }

  store.subscribe((mutation, state) => {
    fnMap[mutation.type](state)
  })
}
