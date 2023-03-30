// Copyright 2021-2023 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

pub(crate) fn config_template() -> &'static str {
    // While using normal toml marshalling would have been way simpler with less overhead,
    // I think it's useful to have comments attached to the saved config file to explain behaviour of
    // particular fields.
    // Note: any changes to the template must be reflected in the appropriate structs.
    r#"
# This is a TOML config file.
# For more information, see https://github.com/toml-lang/toml

##### main base client config options #####

[client]
# Version of the client for which this configuration was created.
version = '{{ client.version }}'

# Human readable ID of this particular client.
id = '{{ client.id }}'

# Indicates whether this client is running in a disabled credentials mode, thus attempting
# to claim bandwidth without presenting bandwidth credentials.
disabled_credentials_mode = {{ client.disabled_credentials_mode }}

# Addresses to nyxd validators via which the client can communicate with the chain.
nyxd_urls = [
    {{#each client.nyxd_urls }}
        '{{this}}',
    {{/each}}
]

# Addresses to APIs running on validator from which the client gets the view of the network.
nym_api_urls = [
    {{#each client.nym_api_urls }}
        '{{this}}',
    {{/each}}
]

# Path to file containing private identity key.
private_identity_key_file = '{{ client.private_identity_key_file }}'

# Path to file containing public identity key.
public_identity_key_file = '{{ client.public_identity_key_file }}'

# Path to file containing private encryption key.
private_encryption_key_file = '{{ client.private_encryption_key_file }}'

# Path to file containing public encryption key.
public_encryption_key_file = '{{ client.public_encryption_key_file }}'

# Path to the database containing bandwidth credentials
database_path = '{{ client.database_path }}'

# Path to the persistent store for received reply surbs, unused encryption keys and used sender tags.
reply_surb_database_path = '{{ client.reply_surb_database_path }}'

##### additional client config options #####

# A gateway specific, optional, base58 stringified shared key used for
# communication with particular gateway.
gateway_shared_key_file = '{{ client.gateway_shared_key_file }}'

# Path to file containing key used for encrypting and decrypting the content of an
# acknowledgement so that nobody besides the client knows which packet it refers to.
ack_key_file = '{{ client.ack_key_file }}'

##### advanced configuration options #####

# Absolute path to the home Nym Clients directory.
nym_root_directory = '{{ client.nym_root_directory }}'

[client.gateway_endpoint]
# ID of the gateway from which the client should be fetching messages.
gateway_id = '{{ client.gateway_endpoint.gateway_id }}'

# Address of the gateway owner to which the client should send messages.
gateway_owner = '{{ client.gateway_endpoint.gateway_owner }}'

# Address of the gateway listener to which all client requests should be sent.
gateway_listener = '{{ client.gateway_endpoint.gateway_listener }}'


##### socket config options #####

[socks5]

# The mix address of the provider to which all requests are going to be sent.
provider_mix_address = '{{ socks5.provider_mix_address }}'

# The port on which the client will be listening for incoming requests
listening_port = {{ socks5.listening_port }}

# Specifies whether this client is going to use an anonymous sender tag for communication with the service provider.
# While this is going to hide its actual address information, it will make the actual communication
# slower and consume nearly double the bandwidth as it will require sending reply SURBs.
#
# Note that some service providers might not support this.
send_anonymously = {{ socks5.send_anonymously }}

##### logging configuration options #####

[logging]

# TODO


##### debug configuration options #####
# The following options should not be modified unless you know EXACTLY what you are doing
# as if set incorrectly, they may impact your anonymity.

# [socks5_debug]


[debug]

[debug.traffic]
average_packet_delay = '{{ debug.traffic.average_packet_delay }}'
message_sending_average_delay = '{{ debug.traffic.message_sending_average_delay }}'

[debug.acknowledgements]
average_ack_delay = '{{ debug.acknowledgements.average_ack_delay }}'

[debug.cover_traffic]
loop_cover_traffic_average_delay = '{{ debug.cover_traffic.loop_cover_traffic_average_delay }}'

"#
}