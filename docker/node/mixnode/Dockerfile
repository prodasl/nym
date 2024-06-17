FROM ubuntu:22.04

#Update the Ubuntu packages to the latest version
RUN apt-get update && \
    apt-get upgrade && \
    apt-get -y install jq wget curl vim telnet

#Create the installation directories
RUN mkdir /opt/nym
RUN mkdir /opt/nym/mix-node

# Install nym-node from the latest binary published
# in github by Nymtech
RUN wget -q -O nym-node \
    $(wget -q -O - 'https://api.github.com/repos/nymtech/nym/releases/latest' |  \
    jq -r '.assets[] | select(.name=="nym-node").browser_download_url') && \
    mv nym-node /opt/nym/mix-node && \
    chmod +x /opt/nym/mix-node/nym-node
ENV PATH="$PATH:/opt/nym/mix-node/"

#Expose the needed port for NYM node operation
EXPOSE 1789 1790 8080

COPY ./node-run.sh /root/node-run.sh
RUN chmod +x /root/node-run.sh

CMD /root/node-run.sh