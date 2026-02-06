FROM kasmweb/firefox:1.14.0

USER root

# Install research tools
RUN apt-get update && apt-get install -y \
    python3-pip \
    python3-venv \
    git \
    curl \
    wget \
    jq \
    vim \
    nano \
    tree \
    htop \
    && rm -rf /var/lib/apt/lists/*

# Install Python research libraries
RUN pip3 install --no-cache-dir \
    requests \
    beautifulsoup4 \
    selenium \
    arxiv \
    scholarly \
    pandas \
    numpy \
    matplotlib \
    jupyter

# Configure Firefox for automation
RUN echo 'user_pref("browser.startup.homepage", "about:blank");' >> /etc/firefox/syspref.js && \
    echo 'user_pref("browser.download.folderList", 2);' >> /etc/firefox/syspref.js && \
    echo 'user_pref("browser.download.dir", "/workspace");' >> /etc/firefox/syspref.js

# Set up workspace directory
RUN mkdir -p /workspace && chown kasm-user:kasm-user /workspace

# Add helper scripts
COPY --chown=kasm-user:kasm-user scripts/research-helper.sh /home/kasm-user/

USER kasm-user
WORKDIR /workspace

# Expose Kasm VNC port
EXPOSE 6901

ENTRYPOINT ["/dockerstartup/kasm_default_profile.sh", "/dockerstartup/vnc_startup.sh", "/dockerstartup/kasm_startup.sh"]
CMD ["--tail-log"]
