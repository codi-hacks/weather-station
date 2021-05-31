import Vue from 'vue'
import VueRouter from 'vue-router'
import Dashboard from '@/views/Dashboard'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'dashboard',
    component: Dashboard
  },
  {
    path: '/about',
    name: 'about',
    // Dynamic import
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/About')
  },
  {
    path: '/stations/:id',
    name: 'station',
    component: () => import(/* webpackChunkName: "station" */ '../views/Station')
    /*
    // Wait for stations to become available before rendering this page
    beforeEnter: (to, from, next) => {
      // Station is available
      if (store.state.stations[to.params.id]) {
        next()
      // Wait until we fetch them from the server
      } else {
        store.state.stationsPromise.then(data => {
          next()
        })
      }
    }
    */
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

export default router
