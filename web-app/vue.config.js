const webpack = require('webpack')

let API_URL = process.env.API_URL

if (!API_URL) {
  try {
    const config = require('./config.json')
    API_URL = config.API_URL
  } catch (e) {
    throw new Error(
      'No config file or environment variables found. ' +
      'Please specify the required environment variables or copy "config.json.example" to "config.json". ' +
      'See the README for more information.'
    )
  }
}

module.exports = {
  chainWebpack: config => {
    config.plugin('html')
      .tap(args => {
        args[0].title = 'Weather Station App'
        return args
      })
  },
  configureWebpack: {
    plugins: [
      new webpack.DefinePlugin({
        API_URL: JSON.stringify(API_URL)
      })
    ]
  },
  pwa: {
    clientsClaim: true,
    skipWaiting: true
  },
  transpileDependencies: [
    'vuetify'
  ]
}
