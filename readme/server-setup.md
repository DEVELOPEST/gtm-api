# Server setup

## Programs to install:
- Docker

## Create production config
```bash
mkdir -p /etc/gtm/gtm-api && cd /etc/gtm/gtm-api
sudo vim Rocket.toml
```
Now paste your config.  
Detailed instructions can be found [here](https://rocket.rs/v0.4/guide/configuration/)  
_You can generate secret with:_
```bash
openssl rand -base64 32
```
