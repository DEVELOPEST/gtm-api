# gtm-api
![Develop](https://github.com/DEVELOPEST/gtm-api/workflows/Develop/badge.svg)
![Deploy](https://github.com/DEVELOPEST/gtm-api/workflows/Deploy/badge.svg)

## Endpoints 

### Commits 
- GET `/commits/<provider>/<user>/<repo>/hash`

### Repositories 
- POST `/repositories`
- PUT `/repositories`
- POST `/repositories/<group_name>/groups`

### Users
- POST `/users`

## Building
**Install dependencies**
```bash
sudo apt install -y libpq-dev
```

**Build package**
```bash
cargo build -p gtm-api
```