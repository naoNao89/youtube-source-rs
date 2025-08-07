# Test Infrastructure Setup Guide

This guide explains how to set up and run the Lavalink integration tests for the YouTube Source Rust plugin.

## Prerequisites

### 1. Docker Installation

The integration tests require Docker to run Lavalink instances. Install Docker for your platform:

- **macOS**: Download Docker Desktop from [docker.com](https://www.docker.com/products/docker-desktop)
- **Windows**: Download Docker Desktop from [docker.com](https://www.docker.com/products/docker-desktop)
- **Linux**: Install using your package manager or from [docker.com](https://docs.docker.com/engine/install/)

### 2. Verify Docker Installation

```bash
# Check if Docker is running
docker --version
docker info

# Check if docker-compose is available
docker-compose --version
```

## Quick Start

### 1. Start Test Environment

```bash
cd test-infrastructure
./run-tests.sh start
```

This will:
- Build the YouTube Source Rust plugin
- Start Lavalink v3 and v4 containers
- Start supporting services (Redis, Prometheus, Grafana)
- Wait for all services to be ready

### 2. Run Integration Tests

```bash
# Run all tests
./run-tests.sh test

# Or run only Rust integration tests
./run-tests.sh rust

# Or run only the test bot
./run-tests.sh bot
```

### 3. Stop Test Environment

```bash
./run-tests.sh stop
```

## Manual Setup

If you prefer to run components manually:

### 1. Build the Plugin

```bash
cd test-infrastructure
cargo build --release
```

### 2. Start Services

```bash
# Start all services
docker-compose up -d

# Or start specific services
docker-compose up -d lavalink-v4 lavalink-v3
```

### 3. Check Service Status

```bash
# Check if services are running
docker-compose ps

# Check Lavalink v4 health
curl http://localhost:2333/version

# Check Lavalink v3 health
curl http://localhost:2334/version
```

### 4. Run Tests

```bash
# From project root
cargo test --features integration-tests lavalink_integration_tests -- --ignored --test-threads=1
```

## Service Configuration

### Lavalink v4 (Port 2333)
- Configuration: `lavalink-v4-config/application.yml`
- Logs: `logs/v4/`
- Health check: `http://localhost:2333/version`

### Lavalink v3 (Port 2334)
- Configuration: `lavalink-v3-config/application.yml`
- Logs: `logs/v3/`
- Health check: `http://localhost:2334/version`

### Additional Services
- **Redis**: Port 6379 (for caching)
- **Prometheus**: Port 9090 (for metrics)
- **Grafana**: Port 3000 (for monitoring dashboards)

## Troubleshooting

### Docker Issues

1. **Docker daemon not running**:
   ```bash
   # Start Docker Desktop (macOS/Windows)
   # Or start Docker service (Linux)
   sudo systemctl start docker
   ```

2. **Permission denied**:
   ```bash
   # Add user to docker group (Linux)
   sudo usermod -aG docker $USER
   # Log out and back in
   ```

3. **Port conflicts**:
   ```bash
   # Check what's using the ports
   lsof -i :2333
   lsof -i :2334
   ```

### Test Failures

1. **Connection refused**: Ensure Lavalink containers are running and healthy
2. **Plugin not found**: Rebuild the plugin with `cargo build --release`
3. **Timeout errors**: Increase wait times in test configuration

### Logs and Debugging

```bash
# View all service logs
./run-tests.sh logs

# View specific service logs
docker-compose logs lavalink-v4
docker-compose logs lavalink-v3

# Follow logs in real-time
docker-compose logs -f lavalink-v4
```

## CI/CD Integration

For automated testing in CI/CD pipelines, ensure:

1. Docker service is available
2. Sufficient memory allocation (minimum 2GB)
3. Network connectivity for downloading dependencies
4. Proper cleanup after tests

Example GitHub Actions setup:
```yaml
- name: Start test infrastructure
  run: |
    cd test-infrastructure
    ./run-tests.sh start

- name: Run integration tests
  run: |
    cd test-infrastructure
    ./run-tests.sh rust
```
