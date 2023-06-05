(function () {
    const __trace_lock = globalThis.__trace_lock;
    const __trace_unlock = globalThis.__trace_unlock;
    
    globalThis.Trace = {
      lock() {
        return __trace_lock();
      },
      unlock() {
        return __trace_unlock();
      },
    };

    globalThis.trace_lock = globalThis.Trace.lock;
    globalThis.trace_unlock = globalThis.Trace.unlock;
  
    Reflect.deleteProperty(globalThis, "__trace_lock");
    Reflect.deleteProperty(globalThis, "__trace_unlock");
  })();
  