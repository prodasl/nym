FROM ubuntu:22.04

#Update the Ubuntu packages to the latest version
RUN apt-get update && \
    apt-get -y upgrade

RUN apt-get -y install jq wget curl vim telnet
RUN apt-get install --reinstall wamerican

#Create the installation directories
RUN mkdir /opt/nym
RUN mkdir /opt/nym/mixnode

# Install nym-node from the latest binary published
# in github by Nymtech
#RUN wget -q -O nym-node \
#    $(wget -q -O - 'https://api.github.com/repos/nymtech/nym/releases/latest' |  \
#    jq -r '.assets[] | select(.name=="nym-node").browser_download_url') && \
#    mv nym-node /opt/nym/mixnode && \
#    chmod +x /opt/nym/mixnode/nym-node

RUN wget https://github.com/nymtech/nym/releases/download/nym-binaries-v2024.6-chomp/nym-node && \
    mv nym-node /opt/nym/mixnode && \
    chmod +x /opt/nym/mixnode/nym-node

ENV PATH="$PATH:/opt/nym/mixnode/"

#Expose the needed port for NYM node operation
EXPOSE 1789 1790 8080 8000

COPY ./node-run.sh /root/node-run.sh
RUN chmod +x /root/node-run.sh

ENTRYPOINT ["bash", "/root/node-run.sh" ]
