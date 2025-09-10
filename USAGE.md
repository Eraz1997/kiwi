# Kiwi User Manual 📚

## First Start 🚀

At the first start, Kiwi won't have any users, not even the administrator. Run

```shell
journalctl --user kiwi.service
```

and you will find a warning log saying something like `admin user not found. invitation created with ID: <id>. please visit https://auth.<your-domain>/create-user?invitation_id=<id>`.

Vist the link and create your admin user, specifying securing credentials. Make sure to never lose or forget them.

Now you can access the admin dashboard at `https://admin.<your-domain>`. From there, I recommend setting up TLS and Dynamic DNS following the instructions just below.

### Dynamic DNS 🐎

> [!NOTE]
> Dynamic DNS currently supports GoDaddy as provider only.

If you are hosting Kiwi on a device with a dynamic public IP address and are using a supported DNS provider (see above), Kiwi lets you set up dynamic DNS to make sure your instance is always online with minimal downtime.

Access the Kiwi admin dashboard, go to the **Dynamic DNS** section and provide Kiwi with your DNS provider API credentials. Since then, Kiwi will periodically check any changes to your instance's public address and will update your DNS records accordingly.

### TLS 🗝️

At the first start, Kiwi generates a self-signed certificate to be able to start communicating with HTTPS from the very beginning. Nonetheless, to ensure correct identity verification, you need to set up a proper certificate trusted by legitimate Certificate Authorities.

Kiwi currently integrates with Let's Encrypt to provide you with TLS management out of the box.

Just head to the **TLS** section of the admin dashboard and order a new certificate from there, then follow the instructions. You'll have to add a verification DNS record with a provided value and ask Kiwi to verify it once done. Some propagation delay might occur.

## Service Integration 🪶

> [!NOTE]
> Kiwi currently supports public Docker images only.

You can create new services inside the **Services** section of the admin dashboard. Each service you add **will be reachable from the Internet** through `https://<service-name>.<your-domain>`.

Services are nothing less than Docker containers, for which you can specify environment variables, secrets, stateful volumes and so on.

For each service, Kiwi creates **a PostgreSQL database with credentials** and **a Redis prefix and user**. Along with custom ones, **your container is given the following environment variables:**

- `KIWI_POSTGRES_URI`, with the URI of the database your service can access, already including username and password
- `KIWI_REDIS_URI`, with the URI of the Redis instance your service can access, already including username and password
- `KIWI_REDIS_PREFIX`, with the prefix of the Redis keys your service can access inside the instance
