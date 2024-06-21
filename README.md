# so-calendar
Interacive TUI for viewing and managing Google calendars

## Tracing

All tracing events are output to `logs/tracing` in the app data folder.

By default tracing events are filtered to `INFO` and above, to change this run the app with `RUST_LOG=trace` environment variable set, where `trace` is the desired log level.

Traces are formatted by the Bunyan formatting layer, for best results when viewing traces install the `bunyan` cli through cargo and run `cat "path_to_tracing_file" | bunyan` for nicely coloured output.
