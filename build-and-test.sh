#!/bin/bash

# Build and test script for your Solana wallet
echo "Building Solana Wallet Docker image..."

# Build the Docker image
docker build -t solana-wallet:latest .

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Docker image built successfully!"
    
    # Run the container for testing
    echo "🚀 Starting container on port 8080..."
    docker run -d \
        --name solana-wallet-test \
        -p 8080:8080 \
        -e SOLANA_RPC_URL=https://api.devnet.solana.com \
        solana-wallet:latest
    
    # Wait a moment for startup
    sleep 3
    
    # Test if the service is running
    echo "🔍 Testing if service is running..."
    if curl -f http://localhost:8080/health 2>/dev/null; then
        echo "✅ Service is running! You can now test with your Next.js frontend."
        echo "📝 Container logs:"
        docker logs solana-wallet-test
    else
        echo "❌ Service health check failed. Check logs:"
        docker logs solana-wallet-test
    fi
    
    echo ""
    echo "🛠️  Useful commands:"
    echo "  View logs: docker logs solana-wallet-test"
    echo "  Stop container: docker stop solana-wallet-test"
    echo "  Remove container: docker rm solana-wallet-test"
    
else
    echo "❌ Docker build failed!"
    exit 1
fi
