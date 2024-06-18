#/bin/bash

# Prompt the user to enter the node id
echo "Please enter the ID you want to set to the node: "
# Get the data from the user input
read node_id
# Validate input has value
if [ -z "$node_id" ]
then
    echo "ERROR: The node ID is mandatory"
    exit 0
fi
# Initialize the node
nym-node run --id $node_id --init-only --mode mixnode --verloc-bind-address 0.0.0.0:1790 --public-ips "$(curl -4 https://ifconfig.me 2>/dev/null)" &> /dev/null
# Run the node
nohup nym-node run --id $node_id --mode mixnode --verloc-bind-address 0.0.0.0:1790 --public-ips "$(curl -4 https://ifconfig.me 2>/dev/nul)" &> /dev/null &
# Show the node details
nym-node node-details --id $node_id