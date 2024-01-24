// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!
$$$$solution_src$$$$
const fs = require('fs')
const code = require('zlib').inflateRawSync(Buffer.from(`$$$$binary_base64$$$$`, 'base64'))
const svc_read_stdio = (fd, buf, count) =>
  fs.readSync(fd, new Uint8Array(exports.memory.buffer, buf, count))
const svc_write_stdio = (fd, buf, count) =>
  fs.writeSync(fd, new Uint8Array(exports.memory.buffer, buf, count))
WebAssembly.instantiate(code, { env: { svc_read_stdio, svc_write_stdio } }).then(
  (wasm) => {
    exports = wasm.instance.exports
    exports._basm_start()
  }
)