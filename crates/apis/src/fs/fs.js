(function () {
    const __writeFileSync = globalThis.__writeFileSync;
    const __readFileSync = globalThis.__readFileSync;
    const __statSync = globalThis.__statSync;
    const __openwrite = globalThis.__openwrite;


    globalThis.Node.FS = {
        readFileChunk(filename, offset, length, flag){
            let data = new Uint8Array(length);

            let n = __readFileSync(
                // just the filename
                filename,
                // The data in which we write the data
                data.buffer,
                data.byteOffset,
                data.byteLength,
                // offset 0
                offset,
                // File write options, like "w" or "a"
                flag,
                // We just ignore the mode, this will run inside the isolated VM, there
                // is no reason to allow other mode
                'utf8'
            );

            return [n, data]
        },
        openwrite(filename, flag){
            return __openwrite(
                // just the filename
                filename,
                flag
            );
        },
        readFileSync(filename, flag){

            let meta = __statSync(filename, 
                flag
                );
            let size = meta.size;
            let data = new Uint8Array(size);

            __readFileSync(
                // just the filename
                filename,
                // The data in which we write the data
                data.buffer,
                data.byteOffset,
                data.byteLength,
                // offset 0
                0,
                // File write options, like "w" or "a"
                flag,
                // We just ignore the mode, this will run inside the isolated VM, there
                // is no reason to allow other mode
                'utf8'
            );

            return data
        },
        statSync(filename, flag){
            return __statSync(
                // just the filename
                filename,
                flag
            );
        },
        listdir(path){
            // TODO
        },
        writeFileSync(filename, data, offset, flag) {
            if (!(data instanceof Uint8Array)) {
                throw TypeError("Data needs to be an Uint8Array");
            }

            console.log("L", data.length)
             
            return __writeFileSync(
                // just the filename
                filename,
                // The actual data
                data.buffer,
                data.byteOffset,
                data.byteLength,
                // We write to 0 offset by default
                offset,
                // File write options, like "w" or "a"
                flag
            );
      },
    };
  
    Reflect.deleteProperty(globalThis, "__writeFileSync");
    Reflect.deleteProperty(globalThis, "__readFileSync");
    Reflect.deleteProperty(globalThis, "__statSync");
    Reflect.deleteProperty(globalThis, "__openwrite");
  })();
  