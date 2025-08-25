# PR Sentinel

A desktop application to manage and monitor your GitHub pull requests with customizable filters.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition)
- [Node Version Manager (nvm)](https://github.com/nvm-sh/nvm)
- [Node.js](https://nodejs.org/) (installed via nvm)

## Quick Start

1. Clone the repository:

```bash
git clone https://github.com/yourusername/pr-sentinel.git
cd pr-sentinel
```

2. Set up the Node.js environment and install dependencies:

```bash
nvm use
npm install
```

3. Start the development environment:

```bash
npm run tauri dev
```

## Development

This project uses:

- [Tauri](https://tauri.app/) for the desktop application framework
- [SvelteKit](https://kit.svelte.dev/) for the frontend
- [TailwindCSS](https://tailwindcss.com/) for styling
