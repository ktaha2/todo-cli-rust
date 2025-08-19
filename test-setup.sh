#!/bin/bash

echo "🦀 RustyTasks - Build and Test Script"
echo "======================================"

# Test Rust backend compilation
echo "📦 Testing Rust backend compilation..."
if cargo check --quiet; then
    echo "✅ Backend compiles successfully"
else
    echo "❌ Backend compilation failed"
    exit 1
fi

# Test individual binary targets
echo "🚀 Testing server binary..."
if cargo check --bin server --quiet; then
    echo "✅ Server binary compiles successfully"
else
    echo "❌ Server binary compilation failed"
fi

echo "💻 Testing CLI binary..."
if cargo check --bin cli --quiet; then
    echo "✅ CLI binary compiles successfully"
else
    echo "❌ CLI binary compilation failed"
fi

# Test frontend if node_modules exists
if [ -d "frontend/node_modules" ]; then
    echo "🎨 Testing frontend TypeScript compilation..."
    cd frontend
    if npx tsc --noEmit --skipLibCheck; then
        echo "✅ Frontend TypeScript compiles successfully"
    else
        echo "❌ Frontend TypeScript compilation failed"
    fi
    cd ..
else
    echo "⚠️ Frontend dependencies not installed. Run 'cd frontend && npm install' to test frontend."
fi

echo ""
echo "🎉 Architecture test complete!"
echo ""
echo "Next steps:"
echo "1. Set up PostgreSQL database"
echo "2. Copy .env.example to .env and configure"
echo "3. Run the database schema: psql -d your_db -f schema.sql"
echo "4. Start the server: cargo run --bin server"
echo "5. In another terminal, start frontend: cd frontend && npm start"
echo "6. Or use the CLI: cargo run --bin cli"