# Build with Docker image

Currently you can build and run in a Docker containers the following the last available Nym node binary:

sudo docker build -t paulrodas/nym-node:latest .

# Running the container
docker run -e NODE_ID=test33 --name **node** -p 8000:8000 -p 8080:8080 -p 1789:1789 -p 1790:1790 -idt paulrodas/nym-node:latest

# Running the Nym node
## Enter to bash terminal in the container
sudo docker exec -it node /bin/bash

## Example to execute commands of an already installed node
nym-node node-details


