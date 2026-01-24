## Contributing

Thanks for your interest in contributing to Stockmate!
To get started, please follow these steps:

1.  **Fork** the repository.
2.  **Clone** your forked repository: `git clone <your-fork-url>`
3.  Install dependencies: `pnpm i`
4.  Start the development server: `pnpm dev`
5.  Make your **edits**.
6.  Run formatting: `pnpm format`
7.  Submit a **Pull Request**.

### Managing Dependency Versions

This project uses a Rust backend (with Tauri) and a Svelte frontend. To keep the application stable, it's important that the dependency versions for these two parts match.

**Guidelines:**

1.  **Rust is the main reference:** The versions in `src-tauri/Cargo.toml` are the correct ones to follow.
2.  **Keep the frontend in sync:** The `tauri` version in Rust and the `@tauri-apps/api` version in `package.json` need to have the same `minor` version number (like `2.9.x`).

**How to update Tauri plugins:**

If you update a Tauri plugin in `src-tauri/Cargo.toml`, you also need to update it in `package.json`. Here is how to do it:

1.  After you change `Cargo.toml`, run a backend build. Then, check the `src-tauri/Cargo.lock` file to see the exact version of the plugin that was installed.
2.  Open `package.json` and find the matching `@tauri-apps/*` package. Set its version to the **exact** number from `Cargo.lock` (for example, `2.7.1`, not `^2.7.1`).

This way, we make sure the frontend and backend use the same tested plugin version.
