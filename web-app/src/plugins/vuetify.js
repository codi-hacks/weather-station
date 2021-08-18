import Vue from 'vue'
import Vuetify from 'vuetify/lib/framework'

Vue.use(Vuetify)

export default new Vuetify({
  theme: {
    options: { customProperties: true },
    themes: {
      blue: {
        accent: '#82B1FF',
        'chart-bg-dark': '#333333',
        'chart-bg-light': '#f3f3f3',
        error: '#FF5252',
        info: '#2196F3',
        primary: '#1976D2',
        secondary: '#424242',
        success: '#4CAF50',
        'text-primary-dark': '#ffffff',
        'text-primary-light': '#000000',
        warning: '#f88C00'
      },
      green: {
        accent: '#B9F6CA',
        'chart-bg-dark': '#333333',
        'chart-bg-light': '#f3f3f3',
        error: '#ff5722',
        info: '#1976D2',
        primary: '#4caf50',
        secondary: '#35330d',
        success: '#009688',
        'text-primary-dark': '#ffffff',
        'text-primary-light': '#000000',
        warning: '#ff9800'
      }
    }
  }
})
