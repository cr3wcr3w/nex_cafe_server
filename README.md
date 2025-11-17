# Nex Cafe Desktop Server

Desktop server for a cyber cafe (PC rental) business. Built with Dioxus 0.7 for Windows desktop.

## Features
- [ ] Authentication
- [ ] Authorization (Role-based access) [Admin/Staff/Customer]
- [ ] Client management (connect, lock, unlock, shutdown)
- [ ] Session management (start, stop, extend, auto-expire)
- [ ] Billing system (rate per hour, session summary)
- [ ] Activity logs and reporting
- [ ] Notifications
- [x] System tray support
  - runs quietly in the system tray
  - Cannot be closed from the UI (only terminable via Task Manager)

## Tech Stack

- Dioxus 0.7.1 (desktop)
- tailwindcss
- axum

## Run (Desktop - Windows)

```bash
dx serve --windows --hot-patch
```
## Useful Commands

Docs:
```bash
cargo doc --open
```

Check unused deps:
```bash
cargo install cargo-udeps
cargo +nightly udeps
```

Bundle:
```bash
dx bundle --package-types "msi" // windows
```

Formatting:
```bash
cargo fmt
dx fmt
```
