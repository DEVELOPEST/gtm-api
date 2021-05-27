<p align="center">
    <img src="./readme/logo.svg" width="256" height="256" alt="logo">
</p>

# GTM backend
![Develop](https://github.com/DEVELOPEST/gtm-api/workflows/Develop/badge.svg)
![Deploy](https://github.com/DEVELOPEST/gtm-api/workflows/Deploy/badge.svg)
  
Backend application GTM time-tracking system.
Publicly hosted at [https://cs.ttu.ee/services/gtm/api](https://cs.ttu.ee/services/gtm/api)

## Endpoints 
Endpoints are documented [here](https://cs.ttu.ee/services/gtm/api/swagger/index.html)

## Building
**Install dependencies**
```bash
sudo apt install -y libpq-dev
```

**Build package**
```bash
cargo build -p gtm-api
```