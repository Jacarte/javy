(function () {
    const __writeFileSync = globalThis.__writeFileSync;
    const __readFileSync = globalThis.__readFileSync;
    const __statSync = globalThis.__statSync;


    globalThis.Node.FS = {
        readFileSync(filename, flag){

            let meta = __statSync(filename, 
                flag
                );
            let size = meta.size;
            let data = new Uint8Array(size);

            return __readFileSync(
                // just the filename
                filename,
                // The data in which we write the data
                data.buffer,
                // offset 0
                0,
                // File write options, like "w" or "a"
                flag,
                // We just ignore the mode, this will run inside the isolated VM, there
                // is no reason to allow other mode
                'utf8'
            );
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
        writeFileSync(filename, data, flag) {
            if (!(data instanceof Uint8Array)) {
                throw TypeError("Data needs to be an Uint8Array");
            }
             
            return __writeFileSync(
                // just the filename
                filename,
                // The actual data
                data.buffer,
                // We write to 0 offset by default
                0,
                // File write options, like "w" or "a"
                flag,
                // We just ignore the mode, this will run inside the isolated VM, there
                // is no reason to allow other mode
                'utf8'
            );
      },
    };
  
    Reflect.deleteProperty(globalThis, "__writeFileSync");
    Reflect.deleteProperty(globalThis, "__readFileSync");
    Reflect.deleteProperty(globalThis, "__statSync");
  })();
  