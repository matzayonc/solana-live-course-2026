# live

Devcontainer environment for the Superteam Poland Solana Bootcamp.

This branch (`master`) provides the pre-configured development environment for the live-coding sessions.

## Getting Started

1. Open this repository in VS Code.
2. When prompted, click **Reopen in Container** (or run `Dev Containers: Reopen in Container` from the Command Palette).
3. The devcontainer includes the full pre-installed Solana toolchain:
   - **Node.js**: 24+
   - **Rust**: 1.95.0
   - **Anchor**: 1.1.2
   - **Surfpool**: local validator
   - **zsh**: configured with autosuggestions and syntax highlighting

## To initialize Anchor project 

```
anchor init --no-git --package-manager npm --test-template mocha diamond-hands
```