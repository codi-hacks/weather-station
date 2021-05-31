import { openDB } from 'idb'

const dbName = 'weather-station'
const storeName = 'default'
const version = 1

export default async store => {
  const db = await openDB(dbName, version, {
    upgrade(db, oldVersion, newVersion, transaction) {
      const idbstore = db.createObjectStore(storeName)
      idbstore.put({}, 'sensors')
      idbstore.put('', 'stations')
    }
  })

  // Alphabetically ordered by key names
  const [sensors, stations] = await db.transaction(storeName)
    .objectStore(storeName).getAll()

  // Avoid writing if the server beat us
  if (!store.state.stations.length) {
    store.commit('setStations', JSON.parse(stations))
  }
  if (!Object.keys(store.state.sensors).length) {
    store.commit('setSensorData', sensors)
  }

  // eslint-disable-next-line no-console
  console.debug('stations hydrated from cache')

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
        idbstore.put(JSON.stringify(state.stations), 'stations')
      })
    },
    setStationsPromise: () => {}
  }

  store.subscribe((mutation, state) => {
    console.log('mutation:', mutation)
    fnMap[mutation.type](state)
  })
}
