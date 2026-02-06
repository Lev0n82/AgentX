FROM kasmweb/ubuntu-jammy-desktop:1.14.0

USER root

# Install development tools
RUN apt-get update && apt-get install -y \
    python3.11 \
    python3.11-venv \
    python3-pip \
    nodejs \
    npm \
    git \
    vim \
    nano \
    emacs \
    tmux \
    build-essential \
    curl \
    wget \
    jq \
    tree \
    htop \
    ripgrep \
    fd-find \
    bat \
    && rm -rf /var/lib/apt/lists/*

# Install VS Code
RUN curl -fsSL https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor -o /usr/share/keyrings/microsoft-archive-keyring.gpg && \
    echo "deb [arch=amd64 signed-by=/usr/share/keyrings/microsoft-archive-keyring.gpg] https://packages.microsoft.com/repos/code stable main" > /etc/apt/sources.list.d/vscode.list && \
    apt-get update && \
    apt-get install -y code && \
    rm -rf /var/lib/apt/lists/*

# Install Rust (for Rust development)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install common Python development packages
RUN pip3 install --no-cache-dir \
    pytest \
    pytest-cov \
    black \
    flake8 \
    mypy \
    ipython \
    jupyter \
    requests \
    fastapi \
    uvicorn

# Install common npm packages globally
RUN npm install -g \
    typescript \
    eslint \
    prettier \
    nodemon

# Set up workspace directory
RUN mkdir -p /workspace && chown kasm-user:kasm-user /workspace

# Add helper scripts
COPY --chown=kasm-user:kasm-user scripts/dev-helper.sh /home/kasm-user/

USER kasm-user

# Install Rust for kasm-user
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/kasm-user/.cargo/bin:${PATH}"

WORKDIR /workspace

# Expose Kasm VNC port
EXPOSE 6901

ENTRYPOINT ["/dockerstartup/kasm_default_profile.sh", "/dockerstartup/vnc_startup.sh", "/dockerstartup/kasm_startup.sh"]
CMD ["--tail-log"]
