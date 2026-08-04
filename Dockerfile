# Reproducible environment for both halves of the repo: the TypeScript
# snapshots in scripts/ and the Anchor workspace in diamond-hands/.
#
#   docker build -t live .
#   docker run --rm -it live                       # anchor test
#   docker run --rm -it live npm --prefix /app/scripts run 1
#
# The base is OtterSec's verified-build image, which already carries the exact
# anchor-cli this workspace targets plus a matching Solana toolchain. Only the
# two things it lacks are added here: Node 24 and Surfpool.

FROM quay.io/ottersec/anchor:v1.1.2 AS toolchain

ENV NODE_VERSION=24 \
    RUST_VERSION=1.95.0 \
    DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        wget \
        git \
        jq \
        ripgrep \
        fd-find \
        bat \
        htop \
        tmux \
        tree \
        fzf \
        build-essential \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# The base ships Node 22 via nvm, but the tests and snapshots run `.ts` files
# through Node's native type stripping, which is only unflagged from Node 23.6.
# Installing over the top and putting /usr/bin first shadows the nvm copy.
RUN curl -fsSL "https://deb.nodesource.com/setup_${NODE_VERSION}.x" | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/* \
    && npm install -g pnpm yarn tsx typescript

ENV PATH=/usr/bin:$PATH

# rust-toolchain.toml pins 1.95.0; pre-install it so the first cargo invocation
# does not stop to download a toolchain.
RUN rustup toolchain install "$RUST_VERSION" --profile minimal \
        --component rustfmt --component clippy \
    && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall -y cargo-expand cargo-edit || true

# Anchor 1.1's default local validator — `anchor test` cannot start without it.
RUN curl -sL https://run.surfpool.run/ | bash
ENV PATH=/root/.local/bin:$PATH

# The Anchor provider needs a wallet keypair; it holds no real funds.
RUN test -f /root/.config/solana/id.json \
    || solana-keygen new --no-bip39-passphrase --silent --outfile /root/.config/solana/id.json

# A usable interactive shell — the container is meant to be worked in, not just
# run once. `--unattended` keeps the installer from starting a nested shell and
# from running `chsh` itself.
RUN apt-get update && apt-get install -y --no-install-recommends \
        zsh \
        less \
        locales \
    && rm -rf /var/lib/apt/lists/* \
    && sed -i 's/^# en_US.UTF-8/en_US.UTF-8/' /etc/locale.gen && locale-gen

# TERM is unset in a bare container, which makes zsh emit malformed colour
# escapes; 256-colour is a safe default that any modern terminal understands.
ENV LANG=en_US.UTF-8 \
    SHELL=/usr/bin/zsh \
    TERM=xterm-256color

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended \
    && git clone --depth 1 https://github.com/zsh-users/zsh-autosuggestions \
        "$HOME/.oh-my-zsh/custom/plugins/zsh-autosuggestions" \
    && git clone --depth 1 https://github.com/zsh-users/zsh-syntax-highlighting \
        "$HOME/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting" \
    && chsh -s /usr/bin/zsh root

COPY .devcontainer/zshrc /root/.zshrc

# Expose Surfpool's RPC port
EXPOSE 8899

CMD ["zsh"]

