(function () {
  const __node_msg = globalThis.__node_msg;
  const __node_node = globalThis.__node_node;
  const __node_send = globalThis.__node_send;
  const __node_done = globalThis.__node_done;
  const __node_msg_length = globalThis.__node_msg_length;
  const __node_node_length = globalThis.__node_node_length;
  const __node_red_result = globalThis.__node_red_result;
  
  globalThis.Node.IO = {
    msg() {
      const buffer = new Uint8Array(__node_msg_length());
      __node_msg(buffer.buffer, buffer.byteOffset, buffer.byteLength);
      let final_decoder = new TextDecoder().decode(buffer)
      return JSON.parse(final_decoder);
    },
    node() {
      const buffer = new Uint8Array(__node_node_length());
      __node_node(buffer.buffer, buffer.byteOffset, buffer.byteLength);
      let final_decoder = new TextDecoder().decode(buffer)
      return JSON.parse(final_decoder);
    },
    send(payload) {
      // TODO Ideally this should be done using quickjs_wasm_rs
      let data = JSON.stringify(payload);
      console.log(data);
      const encodedOutput = new TextEncoder().encode(data);
      const buffer = new Uint8Array(encodedOutput);
      console.log(buffer.buffer, buffer.byteLength);
      return __node_send(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    },
    done(payload) {
      let data = JSON.stringify(payload);
      const encodedOutput = new TextEncoder().encode(data);
      const buffer = new Uint8Array(encodedOutput);
      return __node_done(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    },
    set_result(payload){
      let data =JSON.stringify(payload);
      const encodedOutput = new TextEncoder().encode(data);
      const buffer = new Uint8Array(encodedOutput);
      return __node_red_result(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    }
  };
  globalThis.RED = {
    
  };
  globalThis.RED.util = {
    getMessageProperty(msg, property) {
       console.log("Not implemented globalThis.RED.util.getMessageProperty")
    },
    setMessageProperty(msg, property, val) {
       console.log("Not implemented globalThis.RED.util.setMessageProperty")
    },
  };

  /// This makes easy the port of already existing nodes
  // Create a JS proxy here
  // globalThis.msg = globalThis.Node.IO.msg();
  globalThis.send = globalThis.Node.IO.send;
  globalThis.done = globalThis.Node.IO.done;

  Reflect.deleteProperty(globalThis, "__node_msg");
  Reflect.deleteProperty(globalThis, "__node_send");
  Reflect.deleteProperty(globalThis, "__node_done");
  Reflect.deleteProperty(globalThis, "__node_msg_length");
  Reflect.deleteProperty(globalThis, "__node_node_length");
  Reflect.deleteProperty(globalThis, "__node_node");
  Reflect.deleteProperty(globalThis, "__node_red_result");
})();
