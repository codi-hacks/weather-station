import { openDB } from 'idb'

const dbName = 'weather-station'
const storeName = 'default'
const version = 2

export default async store => {
  const db = await openDB(dbName, version, {
    upgrade(db, existingVersion, newVersion, transaction) {
      // Scorched earth policy
      if (existingVersion > 0 && existingVersion < version) {
        // eslint-disable-next-line no-console
        console.info(`Upgrading idb from ${existingVersion} to ${newVersion}`)
        db.deleteObjectStore(storeName)
      }
      const idbstore = db.createObjectStore(storeName)
      idbstore.put([], 'dashboard')
      idbstore.put({}, 'preferences')
      idbstore.put({}, 'sensors')
      idbstore.put([], 'stations')
    }
  })

  // Alphabetically ordered by key names
  const [dashboard, preferences, sensors, stations] = await db.transaction(storeName)
    .objectStore(storeName).getAll()

  const fnMap = {
    addBookmark: (state, idbstore) => {
      idbstore.delete('dashboard').then(() => {
        idbstore.put(state.dashboard, 'dashboard')
      })
    },
    removeBookmark: (state, idbstore) => {
      idbstore.delete('dashboard').then(() => {
        idbstore.put(state.dashboard, 'dashboard')
      })
    },
    setSensorMode: (state, idbstore) => {
      idbstore.delete('dashboard').then(() => {
        idbstore.put(state.dashboard, 'dashboard')
      })
    },
    setSensorTimeAgo: (state, idbstore) => {
      idbstore.delete('dashboard').then(() => {
        idbstore.put(state.dashboard, 'dashboard')
      })
    },
    setDashboard: (state, idbstore) => {
      idbstore.delete('dashboard').then(() => {
        idbstore.put(state.dashboard, 'dashboard')
      })
    },
    setPreferences: (state, idbstore) => {
      idbstore.delete('preferences').then(() => {
        idbstore.put(state.preferences, 'preferences')
      })
    },
    setSensor: (state, idbstore) => {
      idbstore.delete('sensors').then(() => {
        idbstore.put(state.sensors, 'sensors')
      })
    },
    setStations: (state, idbstore) => {
      idbstore.delete('stations').then(() => {
        idbstore.put(state.stations, 'stations')
      })
    }
  }

  //
  // Hydrate dashboard
  //
  if (dashboard.length) {
    store.commit('setDashboard', dashboard)
  }
  store.state.dashboardPromise.resolve(store.state.dashboard)

  //
  // Hydrate preferences
  //
  if (Object.keys(preferences).length) {
    store.commit('setPreferences', preferences)
  }

  //
  // Hydrate stations
  //
  // Consume the server data if the server response beat us
  if (store.state.stations.length) {
    const tx = db.transaction(storeName, 'readwrite')
    const idbstore = tx.objectStore(storeName)
    fnMap.setStations(store.state, idbstore)
  // Hydrate from the cache if we beat the server response
  } else if (stations && stations.length) {
    // eslint-disable-next-line no-console
    store.commit('setStations', stations)
    console.debug('stations hydrated from cache')
  }

  //
  // Hydrate sensors
  //
  if (Object.keys(store.state.sensors).length) {
    const tx = db.transaction(storeName, 'readwrite')
    const idbstore = tx.objectStore(storeName)
    fnMap.setSensor(store.state, idbstore)
  } else if (stations && stations.length) {
    store.commit('hydrateSensors', { sensors, stations })
    // eslint-disable-next-line no-console
    console.debug('sensors hydrated from cache')
  }

  store.subscribe((mutation, state) => {
    const hook = fnMap[mutation.type]
    if (hook) {
      const tx = db.transaction(storeName, 'readwrite')
      const idbstore = tx.objectStore(storeName)
      hook(state, idbstore)
    }
  })
}
