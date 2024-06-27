#/bin/bash

# Validate input has value
if [ -z "$NODE_ID" ]
then
    NODE_ID=$(shuf -n 1 /usr/share/dict/words | sed "s/'//g")
    echo "NODE_ID=$NODE_ID" >> ~/.bashrc
fi

# run the node
# Initialize the node
CONFIG_FILE="/root/.nym/nym-nodes/"$NODE_ID"/config/config.toml"
if [ ! -f "$CONFIG_FILE" ]; then
    nohup nym-node run --id $NODE_ID --init-only --mode mixnode --verloc-bind-address 0.0.0.0:1790 --public-ips "$(curl -4 https://ifconfig.me 2>/dev/null)" --accept-operator-terms-and-conditions &> /dev/null
fi

nym-node run --mode mixnode --id $NODE_ID --deny-init --accept-operator-terms-and-conditions