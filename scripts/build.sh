#!/bin/bash
set -euo pipefail

# Build script for BadgerGuard

echo "Building BadgerGuard..."

# Build backend services
echo "Building backend services..."
cd backend

echo "  Building shared crate..."
cd shared
cargo build --release
cd ..

echo "  Building gateway..."
cd gateway
cargo build --release
cd ..

echo "  Building auth service..."
cd auth
cargo build --release
cd ..

echo "  Building policy engine..."
cd policy
cargo build --release
cd ..

cd ..

# Build frontend
echo "Building frontend..."
cd frontend/admin
npm ci
npm run build
cd ../..

echo "Build complete!"

