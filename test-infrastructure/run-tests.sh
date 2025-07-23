#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting Lavalink YouTube Rust Plugin Integration Tests${NC}"

# Function to check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        echo -e "${RED}❌ Docker is not running.${NC}"
        echo -e "${YELLOW}Please start Docker Desktop and try again.${NC}"
        echo -e "${BLUE}On macOS/Windows: Start Docker Desktop application${NC}"
        echo -e "${BLUE}On Linux: sudo systemctl start docker${NC}"
        echo ""
        echo -e "${BLUE}For setup instructions, see: test-infrastructure/SETUP.md${NC}"
        exit 1
    fi
    echo -e "${GREEN}✅ Docker is running${NC}"
}

# Function to check if docker-compose is available
check_docker_compose() {
    # Check for Docker Compose v2 first (preferred)
    if command -v docker > /dev/null 2>&1 && docker compose version > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Docker Compose v2 is available${NC}"
        export DOCKER_COMPOSE_CMD="docker compose"
        return 0
    fi

    # Fall back to Docker Compose v1
    if command -v docker-compose > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Docker Compose v1 is available${NC}"
        export DOCKER_COMPOSE_CMD="docker-compose"
        return 0
    fi

    echo -e "${RED}❌ Docker Compose is not installed. Please install it and try again.${NC}"
    echo -e "${YELLOW}Install Docker Compose v2: https://docs.docker.com/compose/install/${NC}"
    exit 1
}

# Function to build the Rust plugin
build_plugin() {
    echo -e "${YELLOW}🔨 Building YouTube Rust plugin...${NC}"
    cd ..
    cargo build --release --all-features
    cd test-infrastructure
    
    # Copy the built library to plugins directory
    mkdir -p plugins
    if [ -f "../target/release/libyoutube_source_rs.so" ]; then
        cp "../target/release/libyoutube_source_rs.so" plugins/
        echo -e "${GREEN}✅ Linux library copied${NC}"
    elif [ -f "../target/release/libyoutube_source_rs.dylib" ]; then
        cp "../target/release/libyoutube_source_rs.dylib" plugins/
        echo -e "${GREEN}✅ macOS library copied${NC}"
    elif [ -f "../target/release/youtube_source_rs.dll" ]; then
        cp "../target/release/youtube_source_rs.dll" plugins/
        echo -e "${GREEN}✅ Windows library copied${NC}"
    else
        echo -e "${YELLOW}⚠️  No native library found, plugin will be downloaded by Lavalink${NC}"
    fi
}

# Function to start the test environment
start_environment() {
    echo -e "${YELLOW}🐳 Starting test environment...${NC}"
    
    # Clean up any existing containers
    ${DOCKER_COMPOSE_CMD} down --remove-orphans

    # Start the services
    ${DOCKER_COMPOSE_CMD} up -d lavalink-v4 lavalink-v3 redis prometheus grafana
    
    echo -e "${YELLOW}⏳ Waiting for services to be ready...${NC}"
    
    # Wait for Lavalink v4
    echo -n "Waiting for Lavalink v4..."
    for i in {1..40}; do
        if curl -s http://localhost:2333/version > /dev/null 2>&1; then
            echo -e " ${GREEN}✅${NC}"
            break
        fi
        if [ $i -eq 40 ]; then
            echo -e " ${RED}❌ Timeout waiting for Lavalink v4${NC}"
            ${DOCKER_COMPOSE_CMD} logs lavalink-v4
            exit 1
        fi
        echo -n "."
        sleep 3
    done

    # Wait for Lavalink v3
    echo -n "Waiting for Lavalink v3..."
    for i in {1..40}; do
        if curl -s http://localhost:2334/version > /dev/null 2>&1; then
            echo -e " ${GREEN}✅${NC}"
            break
        fi
        if [ $i -eq 40 ]; then
            echo -e " ${RED}❌ Timeout waiting for Lavalink v3${NC}"
            ${DOCKER_COMPOSE_CMD} logs lavalink-v3
            exit 1
        fi
        echo -n "."
        sleep 3
    done
    
    echo -e "${GREEN}✅ Test environment is ready${NC}"
}

# Function to run the test bot
run_test_bot() {
    echo -e "${YELLOW}🤖 Running integration test bot...${NC}"
    
    # Build and run the test bot
    ${DOCKER_COMPOSE_CMD} up --build test-bot
    
    echo -e "${GREEN}✅ Test bot completed${NC}"
}

# Function to run Rust integration tests
run_rust_tests() {
    echo -e "${YELLOW}🦀 Running Rust integration tests...${NC}"
    
    cd ..
    
    # Run integration tests with the integration-tests feature
    cargo test --features integration-tests lavalink_integration_tests -- --ignored --test-threads=1
    
    cd test-infrastructure
    
    echo -e "${GREEN}✅ Rust integration tests completed${NC}"
}

# Function to collect test results
collect_results() {
    echo -e "${YELLOW}📊 Collecting test results...${NC}"
    
    # Create results directory
    mkdir -p ../test-results
    
    # Copy test bot results
    if [ -d "test-results" ]; then
        cp -r test-results/* ../test-results/
    fi
    
    # Copy logs
    if [ -d "logs" ]; then
        cp -r logs ../test-results/
    fi
    
    # Generate summary report
    cat > ../test-results/test-summary.md << EOF
# Lavalink YouTube Rust Plugin Test Results

Generated: $(date)

## Test Environment
- Lavalink v4: http://localhost:2333
- Lavalink v3: http://localhost:2334
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3000

## Test Results
$(if [ -f "test-results/integration-test-results.json" ]; then
    echo "### Integration Test Bot Results"
    echo "\`\`\`json"
    cat test-results/integration-test-results.json | jq '.summary'
    echo "\`\`\`"
fi)

## Logs
- Lavalink v4 logs: logs/v4/
- Lavalink v3 logs: logs/v3/

## Monitoring
- Prometheus metrics: http://localhost:9090
- Grafana dashboards: http://localhost:3000 (admin/admin)
EOF
    
    echo -e "${GREEN}✅ Test results collected in ../test-results/${NC}"
}

# Function to cleanup
cleanup() {
    echo -e "${YELLOW}🧹 Cleaning up test environment...${NC}"
    ${DOCKER_COMPOSE_CMD} down --remove-orphans
    echo -e "${GREEN}✅ Cleanup completed${NC}"
}

# Function to validate test environment
validate_environment() {
    echo -e "${YELLOW}🔍 Validating test environment...${NC}"

    # Check if Lavalink containers are running
    if ! ${DOCKER_COMPOSE_CMD} ps | grep -q "lavalink-v4.*Up"; then
        echo -e "${RED}❌ Lavalink v4 container is not running${NC}"
        return 1
    fi

    if ! ${DOCKER_COMPOSE_CMD} ps | grep -q "lavalink-v3.*Up"; then
        echo -e "${RED}❌ Lavalink v3 container is not running${NC}"
        return 1
    fi

    # Check if services are responding
    if ! curl -s http://localhost:2333/version > /dev/null; then
        echo -e "${RED}❌ Lavalink v4 is not responding on port 2333${NC}"
        return 1
    fi

    if ! curl -s http://localhost:2334/version > /dev/null; then
        echo -e "${RED}❌ Lavalink v3 is not responding on port 2334${NC}"
        return 1
    fi

    echo -e "${GREEN}✅ Test environment is valid and ready${NC}"
    return 0
}

# Function to show help
show_help() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  start     Start the test environment only"
    echo "  test      Run all tests (default)"
    echo "  bot       Run only the test bot"
    echo "  rust      Run only Rust integration tests"
    echo "  validate  Validate test environment setup"
    echo "  stop      Stop and cleanup the test environment"
    echo "  logs      Show logs from all services"
    echo "  status    Show status of all services"
    echo "  help      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0              # Run all tests"
    echo "  $0 start        # Start environment for manual testing"
    echo "  $0 validate     # Check if environment is ready"
    echo "  $0 test         # Run all automated tests"
    echo "  $0 stop         # Stop and cleanup"
    echo ""
    echo "For detailed setup instructions, see: SETUP.md"
}

# Function to show logs
show_logs() {
    echo -e "${BLUE}📋 Showing logs from all services...${NC}"
    ${DOCKER_COMPOSE_CMD} logs -f
}

# Function to show status
show_status() {
    echo -e "${BLUE}📊 Service Status:${NC}"
    ${DOCKER_COMPOSE_CMD} ps
    
    echo -e "\n${BLUE}🔗 Service URLs:${NC}"
    echo "Lavalink v4: http://localhost:2333"
    echo "Lavalink v3: http://localhost:2334"
    echo "Prometheus: http://localhost:9090"
    echo "Grafana: http://localhost:3000"
    
    echo -e "\n${BLUE}🏥 Health Checks:${NC}"
    
    # Check Lavalink v4
    if curl -s http://localhost:2333/version > /dev/null 2>&1; then
        echo -e "Lavalink v4: ${GREEN}✅ Healthy${NC}"
    else
        echo -e "Lavalink v4: ${RED}❌ Unhealthy${NC}"
    fi
    
    # Check Lavalink v3
    if curl -s http://localhost:2334/version > /dev/null 2>&1; then
        echo -e "Lavalink v3: ${GREEN}✅ Healthy${NC}"
    else
        echo -e "Lavalink v3: ${RED}❌ Unhealthy${NC}"
    fi
    
    # Check Prometheus
    if curl -s http://localhost:9090/-/healthy > /dev/null 2>&1; then
        echo -e "Prometheus: ${GREEN}✅ Healthy${NC}"
    else
        echo -e "Prometheus: ${RED}❌ Unhealthy${NC}"
    fi
    
    # Check Grafana
    if curl -s http://localhost:3000/api/health > /dev/null 2>&1; then
        echo -e "Grafana: ${GREEN}✅ Healthy${NC}"
    else
        echo -e "Grafana: ${RED}❌ Unhealthy${NC}"
    fi
}

# Main execution
case "${1:-test}" in
    "start")
        check_docker
        check_docker_compose
        build_plugin
        start_environment
        show_status
        ;;
    "test")
        check_docker
        check_docker_compose
        build_plugin
        start_environment
        run_test_bot
        run_rust_tests
        collect_results
        cleanup
        ;;
    "bot")
        check_docker
        check_docker_compose
        run_test_bot
        collect_results
        ;;
    "rust")
        run_rust_tests
        ;;
    "validate")
        check_docker
        check_docker_compose
        validate_environment
        ;;
    "stop")
        cleanup
        ;;
    "logs")
        show_logs
        ;;
    "status")
        show_status
        ;;
    "help")
        show_help
        ;;
    *)
        echo -e "${RED}❌ Unknown command: $1${NC}"
        show_help
        exit 1
        ;;
esac
