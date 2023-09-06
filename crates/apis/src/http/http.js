(function () {
    const __request = globalThis.__request;

    globalThis.Node.HTTP = {
        request(endpoint, data, headers, method){
            return __request(
                endpoint,
                data,
                headers,
                method
            );
        },
    };
  
    Reflect.deleteProperty(globalThis, "__request");
  })();
  