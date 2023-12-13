(function () {
    globalThis.process = {
        version: 'x0.10.0',
        env: {
            DEBUG_MIME: true
        },
        versions: {
            node: "v20.1.0"
        }
    };
    globalThis.module = {};
    globalThis.exports = {};

    globalThis.require = function (name) {

        if(name === "fs") {
            return Node.FS;
        }
        return null;
    }
  })();
  