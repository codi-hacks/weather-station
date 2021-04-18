const dgram = require('dgram')
const server = dgram.createSocket('udp4')

server.on('error', (err) => {
  console.log(`server error:\n${err.stack}`)
  server.close()
})

server.on('message', (msg, rinfo) => {
  console.log(`[${makeTimestamp()}] ${rinfo.address}:${rinfo.port} - ${msg}`)
})

server.on('listening', () => {
  const address = server.address()
  console.log(`server listening ${address.address}:${address.port}`)
})

server.bind(3381)

// Prepend zero to single-digit value
function zeroPad(input) {
  return ('0' + input).slice(-2)
}

// Return string like YYYY-MM-DD HH:MM:SS
function makeTimestamp() {
  const dt = new Date()
  return `${dt.getFullYear()}-${zeroPad(dt.getMonth() + 1)}-${zeroPad(dt.getDate())} ` +
    `${zeroPad(dt.getHours())}:${zeroPad(dt.getMinutes())}`
}
