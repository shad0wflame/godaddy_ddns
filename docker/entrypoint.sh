#!/bin/bash

# Export environment variables.
eval $(printenv | awk -F= '{print "export " "\""$1"\"""=""\""$2"\"" }' >> /etc/profile)

cron -f