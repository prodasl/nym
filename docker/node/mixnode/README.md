# Build with Docker image

Currently you can build and run in a Docker containers the following the last available Nym node binary:

sudo docker build -t paulrodas/nym-node:latest .

# Running the container
sudo docker run --name nym-node-container -p 8080:8080 -p 1789:1789 -p 1790:1790 -idt nym-node sleep infinity

# Running the Nym node
## Enter to bash terminal in the container
sudo docker exec -it nym-node-container /bin/bash

## Go to root directory (contains the node run script)
cd /root
## Run the node and ENTER THE NODE ID in the prompt of the script (E.g. my-first-node)
./node-run.sh


