
This crate wraps the libwing crate for usage in Flutter/Dart apps.

It provides a better interface for dealing with the read() and read_meter()
functions by making them return on a nativePort. This means you won't have to
deal with Isolates in Dart or long-running FFI calls in Flutter (which can
cause reload/restart to hang).

