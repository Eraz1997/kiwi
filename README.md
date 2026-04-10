# Kiwi 🥝

Self-hosted cloud platform for DIY projects, based on Docker containers.

## Installation (Linux Only) 🍓

1. Install Docker ([guide](https://docs.docker.com/engine/install/)).
1. Follow the ["Linux post-installation steps for Docker Engine"](https://docs.docker.com/engine/install/linux-postinstall)
1. Grant Docker permissions to publish on port 443:

    ```shell
    sudo setcap CAP_NET_BIND_SERVICE=+eip $(which docker)
    ```

1. Create a file at `/etc/systemd/system/kiwi.service` with the following content, making sure to replace `<user>` with your user name and `<sha>` with a valid Kiwi Docker image SHA

    ```
    [Unit]
    Description=Kiwi
    After=docker.service
    Requires=docker.service

    [Service]
    User=<user>
    Group=<user>
    TimeoutStartSec=0
    Restart=always
    ExecStartPre=-/usr/bin/docker stop kiwi
    ExecStartPre=-/usr/bin/docker rm kiwi
    ExecStart=/usr/bin/docker run --rm --name kiwi \
        --volume /home/<user>/.kiwi:/config \
        --volume /var/run/docker.sock:/var/run/docker.sock \
        --network host \
        --add-host status.kiwi-local.com:127.0.0.1 \
        --stop-timeout 15 \
        ghcr.io/eraz1997/kiwi@sha256:<sha>
    ExecStop=/usr/bin/docker exec kiwi stop

    [Install]
    WantedBy=default.target
    ```

1. Activate and run the service

    ```shell
    sudo systemctl daemon-reload
    sudo systemctl enable kiwi.service
    sudo systemctl start kiwi.service
    ```

1. Based on your operating system, make sure port 443 is reachable from the Internet at your public IP address.
1. Configure a domain and a wildcard `A` record pointing to your public IP address. Kiwi won't work without a proper initial DNS setup. Do not worry about Dynamic DNS, it's part of the features Kiwi offers.

### If you don't have a public IP address 📢

If your router is behind CG-NAT or anyway doesn't allow you to do port-forwarding, you can leverage Cloudflare Tunnels.

1. Sign up to Cloudflare One with a Free Plan
1. "Onboard your domain", import DNS records automatically and follow instructions
1. Go to `Zero Trust -> Networking -> Connectors -> Tunnels`, create a tunnel and follow installation steps on your machine
1. Open the tunnel configuration in the dashboard and add a route for `Published application` with your domain and `*` as subdomain, `https://localhost:443` as hostname, and `No TLS Verify` enabled.

### Updates 🙃

1. Replace the Docker image `<sha>` with a newer value inside the `/lib/systemd/user/kiwi.service` file
1. Reload and restart the service

    ```shell
    sudo systemctl daemon-reload
    sudo systemctl restart kiwi.service
    ```

## Usage 🚀

Refer to the [user manual](./USAGE.md).

## Development 👨‍💻

Refer to the [development guidelines](./CONTRIBUTING.md).
