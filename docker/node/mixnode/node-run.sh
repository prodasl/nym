#/bin/bash
nym-node run --id $1 --init-only --mode mixnode --verloc-bind-address 0.0.0.0:1790 --public-ips "$(curl -4 https://ifconfig.me)"

nohup nym-node run --id $1 --mode mixnode --verloc-bind-address 0.0.0.0:1790 --public-ips "$(curl -4 https://ifconfig.me)" &