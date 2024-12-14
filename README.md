// I received some assistance from a language model to write this document.

# Rust API with PostgreSQL

A RESTful API built with Rust, Actix-web, and PostgreSQL.
## Complete Setup Guide
### 1. Install Rust
First, you need to install Rust:
```bash
# Windows
# Download and run rustup-init.exe from https://rustup.rs/

# Linux/MacOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After installation, restart your terminal and verify:
```bash
rustc --version
cargo --version
```

### 2. Install PostgreSQL
#### Windows:
Download PostgreSQL installer from official website
Run the installer
Remember your password for 'postgres' user
Note your PostgreSQL installation path (e.g., C:\Program Files\PostgreSQL\17)

#### Linux (Ubuntu/Debian):
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib libpq-dev
```

#### MacOS:
```bash
brew install postgresql
brew services start postgresql
```

### 3. Configure PostgreSQL

1. Start PostgreSQL service:
```bash
# Windows
# PostgreSQL should start automatically

# Linux
sudo service postgresql start

# MacOS
brew services start postgresql
```

2. Create database:
```bash
# Access PostgreSQL
psql -U postgres

# In psql console
CREATE DATABASE users;
\q
```

### 4. Clone and Configure Project

1. Clone the repository:
```bash
git clone https://github.com/muhammadzeejah/rust-api.git
cd rust-api
```

2. Create `.env` file in project root:
```env
DATABASE_URL=postgres://postgres:your_password@localhost:5432/postgres
```
Replace `your_password` with your PostgreSQL password.

### 5. Install Diesel CLI

```bash
# Windows
cargo install diesel_cli --no-default-features --features postgres

# Linux/MacOS
cargo install diesel_cli
```

### 6. Configure PostgreSQL Path

Modify `build.rs`:

```bash
fn main() {
    println!("cargo:rustc-link-search=C:\\Program Files\\PostgreSQL\\17\\lib");
    println!("cargo:rustc-link-lib=libpq");
}
```

### 7. Run Migrations

```bash
diesel setup
diesel migration run
```


### 8. Build and Run

```bash
cargo build
cargo run
```

The API should now be running at `http://localhost:8080`

## API Endpoints

Test the API using curl or Postman:

### Create User
```bash
curl -X POST http://localhost:8080/api/users -H "Content-Type: application/json" -d "{\"name\":\"John Doe\",\"email\":\"john@example.com\"}"
```

### Get All Users
```bash
curl http://localhost:8080/api/users
```

### Get User by ID
```bash
curl http://localhost:8080/api/users/1
```

### Update User
```bash
curl -X PUT http://localhost:8080/api/users/1 \
-H "Content-Type: application/json" \
-d '{"name":"John Updated","email":"john.updated@example.com"}'
```

### Delete User
```bash
curl -X DELETE http://localhost:8080/api/users/1
```

## Troubleshooting

### Common Issues and Solutions

1. **PostgreSQL Connection Error**

   - Verify PostgreSQL is running
   - Check `.env` file configuration
   - Ensure password is correct

2. **Diesel CLI Installation Fails**
   - Ensure PostgreSQL is installed and running
   - Check your PostgreSQL installation path
   - Verify `libpq` is installed

3. **Build Fails**
   - Verify PostgreSQL path in build.rs
   - Check if all dependencies are installed
   - Try `cargo clean` followed by `cargo build`

4. **Migration Errors**
   - Ensure DATABASE_URL is correct
   - Try running diesel setup again
   - Check PostgreSQL user permissions
