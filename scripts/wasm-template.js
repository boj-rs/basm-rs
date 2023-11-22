// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!
const fs = require('fs')
const code = require('zlib').inflateRawSync(Buffer.from(`$$$$binary_base64$$$$`, 'base64'))
const wasm_svc_read_stdio = (fd, buf, count) =>
  fs.readSync(fd, new Uint8Array(wasm_memory.buffer, buf, count))
const wasm_svc_write_stdio = (fd, buf, count) =>
  fs.writeSync(fd, new Uint8Array(wasm_memory.buffer, buf, count))
WebAssembly.instantiate(code, { env: { wasm_svc_read_stdio, wasm_svc_write_stdio } }).then(
  (wasm) => {
    const { _start, memory } = wasm.instance.exports
    wasm_memory = memory
    _start()
  }
)

$$$$solution_src$$$$