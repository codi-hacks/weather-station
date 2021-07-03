export default class Deferred {
  constructor() {
    this.promise = new Promise((resolve, reject) => {
      this._reject = reject
      this._resolve = resolve
    })
  }

  resolve(payload) {
    if (this._resolve) {
      this._resolve(payload)
    } else {
      const intervalId = setInterval(() => {
        console.log('checking for resolve')
        if (this._resolve) {
          clearInterval(intervalId)
          this._resolve(payload)
        }
      }, 10)
    }
  }

  reject(reason) {
    if (this._resolve) {
      this._resolve(reason)
    } else {
      const intervalId = setInterval(() => {
        console.log('checking for reject')
        if (this._resolve) {
          clearInterval(intervalId)
          this._resolve(reason)
        }
      }, 10)
    }
  }
}
