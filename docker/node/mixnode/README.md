# Build with Docker image

Currently you can build and run in a Docker containers the following the last available Nym node binary:

sudo docker build -t paulrodas/nym-node:latest .

This image can be used to run a nym mixnode version 1.1.3

# Running the container wit A RAMDON ID
docker run --name nym-node -p 8000:8000 -p 8080:8080 -p 1789:1789 -p 1790:1790 -idt paulrodas/nym-node:latest

### OPTIONAL Running the container wit A SPECIFIED ID
docker run --name nym-node -e NODE_ID=customnodeid -p 8000:8000 -p 8080:8080 -p 1789:1789 -p 1790:1790 -idt paulrodas/nym-node:latest

# Monitoring the node
docker logs nym-node --follow

# Running the Nym node
## Enter to bash terminal in the container to admin it
docker exec -it nym-node /bin/bash
## Running nym-node commands inside the container (NODE_ID environment variable ALREADY has the ID set to the node)
nym-node node-details --id $NODE_ID

