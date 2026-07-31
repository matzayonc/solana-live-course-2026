# live

Live-coding demos for the Superteam Poland Solana Bootcamp — the runnable
companion to the decks in [`../dynaprez`](../dynaprez).

## Summary

Two halves, independent of each other:

```
scripts/         client-side snapshots (TypeScript, @solana/kit)
diamond-hands/   on-chain program (Anchor 1.1, mocha tests)
```

**`scripts/`** is the live-coding path, built as *snapshots*. A snapshot is one
file: a complete, runnable program at one point in the story. No shared `lib/`,
no imports between snapshots — duplication is deliberate, so any snapshot can be
opened cold, read top to bottom, and run.

- **No dead air.** If live typing goes wrong, `npm run 2` is a working program.
- **The diff is the lesson.** `diff 1_keypair.ts 2_balance.ts` is exactly what
  gets typed live.
- **Numbered, one script each.** `npm run 1`, `npm run 2`, …
- **Any entry point.** Start at snapshot 3 if the room already knows the rest.

| # | Snapshot | Point |
|---|----------|-------|
| 1 | `1_keypair` | a keypair is just keys; the address derives from it |
| 2 | `2_balance` | load a keypair from disk, read a token balance |
| 3 | `3_transfer` | sign and send a SOL transfer (devnet) |
| 4 | *quote* | ask Jupiter for a price |
| 5 | *swap* | build, sign and send the swap |

**`diamond-hands/`** is a token time-lock: `lock(amount, duration)` moves tokens
into a program-owned vault, and `withdraw()` returns them only once the lock has
expired, closing the vault and refunding its rent. Deposits live in a PDA at
`[b"lock", owner, mint]`; the vault's authority *is* that PDA, so tokens can only
move in a transaction this program signs for.

## How to run

The two halves install separately.

```bash
cd scripts
npm install
npm run 1        # run snapshot 1; npm run 2, npm run 3, …
```

Run snapshots from `scripts/` — they read and write `./keypair.json` relative to
the working directory.

```bash
cd diamond-hands
npm install      # anchor test does NOT do this for you
anchor test      # builds, starts surfpool, deploys, runs tests/
```

`anchor test` handles the whole cycle after that first `npm install`: ~100s from
clean, ~3s on a rerun. Useful variations:

```bash
anchor test --skip-build          # tests only, when the program is unchanged
anchor build                      # program + IDL + target/types/
npx tsc --noEmit                  # typecheck — mocha strips types, never checks them
```

### In a container

Nothing but Docker needed — the image carries the whole toolchain.

```bash
docker build -t live .
docker run --rm live                          # anchor test
docker run --rm live npm --prefix /app/scripts run 1
```

The base is [`quay.io/ottersec/anchor:v1.1.2`](https://quay.io/repository/ottersec/anchor),
the verified-build image, so anchor-cli and the Solana toolchain match what
on-chain verification uses. It ships Node 22, which is too old to run `.ts`
files natively, so the Dockerfile installs Node 24 over it and adds Surfpool.

VS Code users can instead **Reopen in Container** — `.devcontainer/` builds the
same Dockerfile but stops at the `toolchain` stage and bind-mounts the source,
so edits land on the host. It forwards port 8899 and caches `~/.cargo/registry`
and `~/.npm` in volumes across rebuilds.

The shell is zsh with oh-my-zsh (autosuggestions and syntax highlighting), set
up in `.devcontainer/zshrc`. It defines a few shortcuts, since the repo has two
halves and the commands differ per half:

| | |
|---|---|
| `dh` / `sc` | jump to `diamond-hands/` or `scripts/` |
| `t` / `tb` | `anchor test` / `anchor test --skip-build` |
| `b` / `tc` | `anchor build` / `npx tsc --noEmit` |

`ws` returns to the repo root. It follows `LIVE_ROOT`, which is `/app` in the
image and the mounted workspace in the devcontainer.

## Requirements

Only needed when running outside the container:

| | |
|---|---|
| Node | 24+ |
| Anchor | 1.1.2 (`avm install 1.1.2 && avm use 1.1.2`) |
| Rust | 1.95.0 — pinned in `rust-toolchain.toml` |
| Surfpool | on `PATH`: `curl -sL https://run.surfpool.run/ \| bash` |

Node runs `.ts` files directly by stripping the types out — no build step, no
`tsx` and no `ts-mocha`. It prints an `ExperimentalWarning` about that on every
run, silenced by `--disable-warning=ExperimentalWarning`. Types are never
checked at runtime; run `npx tsc --noEmit` separately.

**Surfpool is Anchor 1.1's default local validator** (`anchor test --validator
legacy` falls back to `solana-test-validator`). It must be on `PATH` and port
8899 must be free, or Anchor exits with `port 8899 … already in use` — set
`[surfpool] rpc_port = N` in `Anchor.toml` to move it.

Do not switch to `--validator legacy`: Anchor runs surfpool with
`--block-production-mode transaction`, so blocks are produced only when a
transaction arrives and the clock is frozen in between. The test suite jumps the
clock with the `surfnet_timeTravel` cheatcode instead of sleeping, which turns
the expiry test from ~90 seconds of unpredictable waiting into ~200ms. That
cheatcode does not exist on `solana-test-validator`. Its parameter is in
**milliseconds** while the `Clock` sysvar is in seconds, and it refuses to
travel backwards.

For the `scripts/` snapshots, the public RPC endpoints are rate-limited and will
get flaky in front of an audience — swap in a Helius/Triton endpoint before
presenting. The devnet faucet is frequently dry, so fund the keypair ahead of
time via <https://faucet.solana.com> rather than relying on `client.airdrop`.

## Agent skill

The Solana agent skill lives in `diamond-hands/.agents/skills/solana-dev`,
symlinked into `.claude/skills/`. Anchor's `--install-agent-skills` flag is
broken as of anchor-cli 1.1.2 — it calls `npx skills add --skill <url>`, but the
CLI wants the URL as a positional argument, and the failure is swallowed.
Install it by hand:

```bash
npx skills add https://github.com/solana-foundation/solana-dev-skill
```
