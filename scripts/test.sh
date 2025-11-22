#!/bin/bash
set -euo pipefail

# Test script for BadgerGuard

echo "Running tests..."

# Test backend
echo "Testing backend services..."
cd backend
cargo test --workspace
cd ..

# Test frontend
echo "Testing frontend..."
cd frontend/admin
npm test -- --run
cd ../..

echo "Tests complete!"

