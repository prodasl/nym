#/bin/bash

# Validate input has value
if [ -z "$NODE_ID" ]
then
    echo "ERROR: The environment variable NODE_ID must be set in the docker run command"
    exit 0
fi

# run the node
# Initialize the node
CONFIG_FILE="/root/.nym/nym-nodes/"$NODE_ID"/config/config.toml"
if [ ! -f "$CONFIG_FILE" ]; then
    nohup nym-node run --id $NODE_ID --init-only --mode mixnode --verloc-bind-address 0.0.0.0:1790 --public-ips "$(curl -4 https://ifconfig.me 2>/dev/null)" &> /dev/null
fi

nym-node run --mode mixnode --id $NODE_ID --deny-init