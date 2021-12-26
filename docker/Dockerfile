FROM rust:1.57 as builder
WORKDIR /usr/src/godaddy_ddns
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

# Install dependencies.
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev cron &&  \
    rm -rf /var/lib/apt/lists/*

# Add application from builder container.
COPY --from=builder /usr/local/cargo/bin/godaddy_ddns /usr/local/bin/godaddy_ddns

# Add crontab
COPY --from=builder /usr/src/godaddy_ddns/docker/crontab /etc/cron.d/godaddy_ddns
RUN chmod 0644 /etc/cron.d/godaddy_ddns
RUN crontab /etc/cron.d/godaddy_ddns

# Add entrypoint
COPY --from=builder /usr/src/godaddy_ddns/docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

CMD ["/entrypoint.sh"]