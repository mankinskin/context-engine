**Memory & Cleanup**
- [ ] No unbounded `Closure::forget()` calls; use `Closure::into_js_value()` to
      transfer ownership to the JS GC instead.
- [ ] Document-level event listeners registered with a `on_cleanup` removal hook
      so they are unregistered if the component unmounts mid-gesture.
- [ ] No `Rc`/`RefCell` or wasm-bindgen closures that outlive component scope
      without an explicit cleanup path.