(function () {
  const __node_msg = globalThis.__node_msg;
  const __node_send = globalThis.__node_send;
  const __node_done = globalThis.__node_done;

  globalThis.Node.IO = {
    msg() {
      // let buff = [];
      // let read = __node_msg(buff);
      // let data = buff[..read]
      return JSON.parse(__node_msg());
    },
    send(payload) {

      let data = JSON.stringify(payload);
      const encodedOutput = new TextEncoder().encode(data);
      const buffer = new Uint8Array(encodedOutput);
      console.log(buffer.buffer, buffer.offset, buffer.length);
      return __node_send(buffer.buffer, buffer.offset, buffer.length);
    },
    done(payload) {
      let data = JSON.stringify(payload);
      const encodedOutput = new TextEncoder().encode(data);
      const buffer = new Uint8Array(encodedOutput);
      return __node_done(buffer.buffer, buffer.offset, buffer.length);
    }
  };

  Reflect.deleteProperty(globalThis, "__node_msg");
  Reflect.deleteProperty(globalThis, "__node_send");
  Reflect.deleteProperty(globalThis, "__node_done");
})();
