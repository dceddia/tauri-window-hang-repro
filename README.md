# Tauri Panics with a Tao Window on Windows

This repo demonstrates an issue where creating a second native window (using `create_tao_window`)
causes the app to panic if you click anywhere inside the app, or do anything that sends it a window event (minimize, unfocus, etc).

The stack trace is below.

I think the relevant part is #5 - `tauri-runtime-wry-0.3.5\src\lib.rs:2447`. That section looks like it's handling window events, and makes the assumption that an event will definitely be from a window with a webview.

The `webview_id_map`, too, assumes that a call to `get()` will find an associated window, and unwraps the Option, which causes the panic.

```
Event::WindowEvent {
    event, window_id, ..
} => {
    let window_id = webview_id_map.get(&window_id);

    ...
```

```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tauri-runtime-wry-0.3.5\src\lib.rs:128:36
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\/library\std\src\panicking.rs:498
   1: core::panicking::panic_fmt
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\/library\core\src\panicking.rs:107
   2: core::panicking::panic
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\/library\core\src\panicking.rs:48
   3: enum$<core::option::Option<ref$<u64> >, 1, 18446744073709551615, Some>::unwrap<ref$<u64> >
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\core\src\option.rs:746
   4: tauri_runtime_wry::WebviewIdStore::get
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tauri-runtime-wry-0.3.5\src\lib.rs:128
   5: tauri_runtime_wry::handle_event_loop<enum$<tauri::EventLoopMessage> >
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tauri-runtime-wry-0.3.5\src\lib.rs:2447
   6: tauri_runtime_wry::impl$49::run::closure$0<enum$<tauri::EventLoopMessage>,tauri::app::impl$17::run::closure$0>
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tauri-runtime-wry-0.3.5\src\lib.rs:1954
   7: tao::platform_impl::platform::event_loop::impl$2::run_return::closure$0<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > >,tauri_runtime_wry::impl$49::run::closure$0>
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop.rs:225
   8: alloc::boxed::impl$45::call_mut<tuple$<enum$<tao::event::Event<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > > >,ref_mut$<enum$<tao::event_loop::ControlFlow> > >,dyn$<core::ops::function::FnMut<tuple$<enum$<tao::event::Event<enum$<ta
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\alloc\src\boxed.rs:1701
   9: tao::platform_impl::platform::event_loop::runner::impl$3::call_event_handler::closure$0<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop\runner.rs:249
  10: core::panic::unwind_safe::impl$23::call_once<tuple$<>,tao::platform_impl::platform::event_loop::runner::impl$3::call_event_handler::closure$0>
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\core\src\panic\unwind_safe.rs:271
  11: std::panicking::try::do_call<core::panic::unwind_safe::AssertUnwindSafe<tao::platform_impl::platform::event_loop::runner::impl$3::call_event_handler::closure$0>,tuple$<> >
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\std\src\panicking.rs:406
  12: core::option::impl$41::from_residual<tuple$<alloc::string::String,alloc::string::String> >
  13: std::panicking::try<tuple$<>,core::panic::unwind_safe::AssertUnwindSafe<tao::platform_impl::platform::event_loop::runner::impl$3::call_event_handler::closure$0> >
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\std\src\panicking.rs:370
  14: std::panic::catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tao::platform_impl::platform::event_loop::runner::impl$3::call_event_handler::closure$0>,tuple$<> >
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\std\src\panic.rs:133
  15: tao::platform_impl::platform::event_loop::runner::EventLoopRunner<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >::catch_unwind<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > >,tuple$<>,tao::platform_impl::platform
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop\runner.rs:155
  16: tao::platform_impl::platform::event_loop::runner::EventLoopRunner<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >::call_event_handler<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop\runner.rs:241
  17: tao::platform_impl::platform::event_loop::runner::EventLoopRunner<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >::send_event<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop\runner.rs:223
  18: tao::platform_impl::platform::event_loop::SubclassInput<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >::send_event<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop.rs:107
  19: tao::platform_impl::platform::event_loop::public_window_callback_inner::closure$6<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop.rs:1632
  20: core::ops::function::FnOnce::call_once<tao::platform_impl::platform::event_loop::public_window_callback_inner::closure$6,tuple$<> >
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\core\src\ops\function.rs:227
  21: core::panic::unwind_safe::impl$23::call_once<tuple$<>,tao::platform_impl::platform::event_loop::public_window_callback_inner::closure$6>
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\core\src\panic\unwind_safe.rs:271
  22: std::panicking::try::do_call<core::panic::unwind_safe::AssertUnwindSafe<tao::platform_impl::platform::event_loop::public_window_callback_inner::closure$6>,tuple$<> >
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\std\src\panicking.rs:406
  23: core::option::impl$41::from_residual<tuple$<alloc::string::String,alloc::string::String> >
  24: std::panicking::try<tuple$<>,core::panic::unwind_safe::AssertUnwindSafe<tao::platform_impl::platform::event_loop::public_window_callback_inner::closure$6> >
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\std\src\panicking.rs:370
  25: std::panic::catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tao::platform_impl::platform::event_loop::public_window_callback_inner::closure$6>,tuple$<> >
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\std\src\panic.rs:133
  26: tao::platform_impl::platform::event_loop::runner::EventLoopRunner<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >::catch_unwind<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > >,tuple$<>,tao::platform_impl::platform
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop\runner.rs:155
  27: tao::platform_impl::platform::event_loop::public_window_callback_inner<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop.rs:1978
  28: tao::platform_impl::platform::event_loop::public_window_callback<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop.rs:848
  29: DefSubclassProc
  30: DefSubclassProc
  31: CallWindowProcW
  32: DispatchMessageW
  33: SendMessageTimeoutW
  34: KiUserCallbackDispatcher
  35: NtUserGetMessage
  36: GetMessageW
  37: windows::Windows::Win32::UI::WindowsAndMessaging::GetMessageW<windows::Windows::Win32::Foundation::HWND>
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\windows-0.30.0\src\Windows\Win32\UI\WindowsAndMessaging\mod.rs:4336
  38: tao::platform_impl::platform::event_loop::EventLoop<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >::run_return<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > >,tauri_runtime_wry::impl$49::run::closure$0>
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop.rs:236
  39: tao::platform_impl::platform::event_loop::EventLoop<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >::run<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > >,tauri_runtime_wry::impl$49::run::closure$0>
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\platform_impl\windows\event_loop.rs:209
  40: tao::event_loop::EventLoop<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > > >::run<enum$<tauri_runtime_wry::Message<enum$<tauri::EventLoopMessage> > >,tauri_runtime_wry::impl$49::run::closure$0>
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tao-0.7.0\src\event_loop.rs:177
  41: tauri_runtime_wry::impl$49::run<enum$<tauri::EventLoopMessage>,tauri::app::impl$17::run::closure$0>
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tauri-runtime-wry-0.3.5\src\lib.rs:1953
  42: tauri::app::App<tauri_runtime_wry::Wry<enum$<tauri::EventLoopMessage> > >::run<tauri_runtime_wry::Wry<enum$<tauri::EventLoopMessage> >,app::main::closure$0>
             at C:\Users\Dave\.cargo\registry\src\github.com-1ecc6299db9ec823\tauri-1.0.0-rc.6\src\app.rs:614
  43: app::main
             at .\src\main.rs:15
  44: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\library\core\src\ops\function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[0410/215004.211:ERROR:window_impl.cc(114)] Failed to unregister class Chrome_WidgetWin_0. Error = 0
```