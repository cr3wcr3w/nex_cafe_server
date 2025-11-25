# Nex Cafe Desktop Server

Desktop server for a cyber cafe (PC rental) business. Built with Dioxus 0.7 for Windows desktop.

## Features
- [x] Authentication
- [ ] Authorization (Role-based access) [Admin/Staff/Customer]
- [ ] Client management (add time, restart, shutdown, reset password)
- [ ] Billing system
- [ ] Activity logs and reporting
- [ ] Notifications
- [ ] Point Based reward system
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

Cleaning build cache:
```bash
cargo clean
```