(function () {
  const __node_msg = globalThis.__node_msg;
  const __node_send = globalThis.__node_send;
  const __node_done = globalThis.__node_done;
  const __node_msg_length = globalThis.__node_msg_length;
  globalThis.Node.IO = {
    msg() {
        const buffer = new Uint8Array(__node_msg_length());
        // Stdin file descriptor
      // let buff = [];
      // let read = __node_msg(buff);
      // let data = buff[..read]
      return JSON.parse(__node_msg(buffer.buffer, buffer.byteOffset, buffer.byteLength));
    },
    send(payload) {

      let data = JSON.stringify(payload);
      const encodedOutput = new TextEncoder().encode(data);
      const buffer = new Uint8Array(encodedOutput);
      console.log(buffer.buffer, buffer.offset, buffer.length);
      return __node_send(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    },
    done(payload) {
      let data = JSON.stringify(payload);
      const encodedOutput = new TextEncoder().encode(data);
      const buffer = new Uint8Array(encodedOutput);
      return __node_done(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    }
  };

  Reflect.deleteProperty(globalThis, "__node_msg");
  Reflect.deleteProperty(globalThis, "__node_send");
  Reflect.deleteProperty(globalThis, "__node_done");
  Reflect.deleteProperty(globalThis, "__node_msg_length");
})();
