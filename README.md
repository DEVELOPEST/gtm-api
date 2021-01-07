# gtm-api

## Endpoints 

### Commits 
- GET `/commits/<provider>/<username>/<repo>/hash`

### Repositories 
- POST `/repositories`
- PUT `/repositories`

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