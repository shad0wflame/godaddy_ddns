########################################################################################################################
## Builder
########################################################################################################################
FROM rust:1.57 as builder

RUN update-ca-certificates

WORKDIR /usr/src

COPY . .

RUN cargo build --release
RUN strip -s /usr/src/target/release/godaddy_ddns

########################################################################################################################
## Final image
########################################################################################################################
FROM debian:bullseye-slim

# Install dependencies.
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev cron &&  \
    rm -rf /var/lib/apt/lists/*

# Add application from builder container.
COPY --from=builder /usr/src/target/release/godaddy_ddns /usr/local/bin/godaddy_ddns

# Add crontab
COPY --from=builder /usr/src/docker/crontab /etc/cron.d/godaddy_ddns
RUN chmod 0644 /etc/cron.d/godaddy_ddns
RUN crontab /etc/cron.d/godaddy_ddns

# Add entrypoint
COPY --from=builder /usr/src/docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

CMD ["/entrypoint.sh"]