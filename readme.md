# Time Tracker

Silently logs the active application title and path to a file so you can review time spent on projects each day.

- Windows only (for now)
- Output is NDJSON
- Polls every second
- Idle timeout is 60 seconds
- Saves in `~/TimeTracker/YYYY-MM-DD.ndjson`

## TODO

- Configurable polling rate, idle timeout
- Write final line when exiting
- Other file formats (CSV)
- macOS bindings
- Rotate to new log file if running at midnight (?)
