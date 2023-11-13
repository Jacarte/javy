(function () {
  const __node_msg = globalThis.__node_msg;
  const __node_node = globalThis.__node_node;
  const __node_send = globalThis.__node_send;
  const __node_done = globalThis.__node_done;
  const __node_msg_length = globalThis.__node_msg_length;
  const __node_pop = globalThis.__node_pop;
  const __node_pop_length = globalThis.__node_pop_length;
  const __node_node_length = globalThis.__node_node_length;
  const __node_red_result = globalThis.__node_red_result;
  const __node_red_register = globalThis.__node_red_register;
  const __node_warn = globalThis.__node_warn;
  const __node_context = globalThis.__node_context;
  const __node_context_length = globalThis.__node_context_length;
  const __node_emit = globalThis.__node_emit;
  
  globalThis.Node.IO = {
    emit(event, payload) {
      let data = JSON.stringify({ event: event, payload: payload });
      console.log(data);
      const encodedOutput = new TextEncoder().encode(data);
      const buffer = new Uint8Array(encodedOutput);
      console.log(buffer.buffer, buffer.byteLength);
      return __node_emit(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    },
    msg() {
      const buffer = new Uint8Array(__node_msg_length());
      __node_msg(buffer.buffer, buffer.byteOffset, buffer.byteLength);
      let final_decoder = new TextDecoder().decode(buffer)
      return JSON.parse(final_decoder);
    },
    pop() {
      const buffer = new Uint8Array(__node_pop_length());
      __node_pop(buffer.buffer, buffer.byteOffset, buffer.byteLength);
      let final_decoder = new TextDecoder().decode(buffer)
      console.log(final_decoder);
      return JSON.parse(final_decoder);
    },
    context() {
      const buffer = new Uint8Array(__node_context_length());
      __node_context(buffer.buffer, buffer.byteOffset, buffer.byteLength);
      let final_decoder = new TextDecoder().decode(buffer)
      return JSON.parse(final_decoder);
    },
    node() {
      const buffer = new Uint8Array(__node_node_length());
      console.log("[WASM] buffer size", buffer.length);
      __node_node(buffer.buffer, buffer.byteOffset, buffer.byteLength);
      let final_decoder = new TextDecoder().decode(buffer)
      return JSON.parse(final_decoder);
    },
    warn(payload){

      let data = JSON.stringify(payload);
      console.log(data);
      const encodedOutput = new TextEncoder().encode(data);
      const buffer = new Uint8Array(encodedOutput);
      console.log(buffer.buffer, buffer.byteLength);
      return __node_warn(buffer.buffer, buffer.byteOffset, buffer.byteLength);
      
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
    },
    register_type(name, constructor, options){
       let data = JSON.stringify({ name: name, constructor: constructor, options: options });
       const encodedOutput = new TextEncoder().encode(data);
       const buffer = new Uint8Array(encodedOutput);
       return __node_red_register(buffer.buffer, buffer.byteOffset, buffer.byteLength)
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
  globalThis.warn = globalThis.Node.IO.warn;

  Reflect.deleteProperty(globalThis, "__node_msg");
  Reflect.deleteProperty(globalThis, "__node_send");
  Reflect.deleteProperty(globalThis, "__node_done");
  Reflect.deleteProperty(globalThis, "__node_msg_length");
  Reflect.deleteProperty(globalThis, "__node_red_register");
  Reflect.deleteProperty(globalThis, "__node_node_length");
  Reflect.deleteProperty(globalThis, "__node_node");
  Reflect.deleteProperty(globalThis, "__node_red_result");
  Reflect.deleteProperty(globalThis, "__node_warn");
  Reflect.deleteProperty(globalThis, "__node_context");
  Reflect.deleteProperty(globalThis, "__node_context_length");
  Reflect.deleteProperty(globalThis, "__node_emit");
})();
