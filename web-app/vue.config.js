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
  transpileDependencies: [
    'vuetify'
  ],
  pwa: {
    themeColor: "#2196F3",
    name: "Weather Station",
    iconPaths: [
      {
        "src": "./public/img/icons/android-chrome-192x192.png",
        "sizes": "192x192",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/android-chrome-512x512.png",
        "sizes": "512x512",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/android-chrome-maskable-192x192.png",
        "sizes": "192x192",
        "type": "image/png",
        "purpose": "maskable"
      },
      {
        "src": "./public/img/icons/android-chrome-maskable-512x512.png",
        "sizes": "512x512",
        "type": "image/png",
        "purpose": "maskable"
      },
      {
        "src": "./public/img/icons/apple-touch-icon-60x60.png",
        "sizes": "60x60",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/apple-touch-icon-76x76.png",
        "sizes": "76x76",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/apple-touch-icon-120x120.png",
        "sizes": "120x120",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/apple-touch-icon-152x152.png",
        "sizes": "152x152",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/apple-touch-icon-180x180.png",
        "sizes": "180x180",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/apple-touch-icon.png",
        "sizes": "180x180",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/favicon-16x16.png",
        "sizes": "16x16",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/favicon-32x32.png",
        "sizes": "32x32",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/msapplication-icon-144x144.png",
        "sizes": "144x144",
        "type": "image/png"
      },
      {
        "src": "./public/img/icons/mstile-150x150.png",
        "sizes": "150x150",
        "type": "image/png"
      }
    ]
  }
}
