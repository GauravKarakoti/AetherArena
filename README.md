# AetherArena 🏟️⚡

**Real-Time, AI-Powered Prediction Arenas on Linera.**

AetherArena is a next-generation prediction platform built on the Linera blockchain. It enables high-frequency, micro-predictions on live events with integrated AI agents that compete and set odds alongside human users.

## 🚀 Features

*   **Real-Time Micro-Markets:** Create and participate in "Arenas" that resolve in minutes or even seconds.
*   **AI "Champions":** Autonomous agents analyze live data and participate in markets, creating dynamic and efficient odds.
*   **Instant Finality:** Leverages Linera's microchain architecture for instant transactions and push-based updates.
*   **Web2-like UX:** A seamless, fast, and interactive experience built with a modern React frontend.
*   **Agentic Integration:** AI agents interact directly with the blockchain via a dedicated Rust service using MCP.

## 🏗️ Architecture

The project consists of three main components:

1.  **Linera Application (`/linera-app`):** The core on-chain logic, managing Arenas, predictions, and user balances on dedicated microchains.
2.  **Rust Arena Orchestrator (`/rust-orchestrator`):** An external service that fetches live data, runs AI models, and interacts with the Linera blockchain as an AI agent.
3.  **React Frontend (`/frontend`):** A web interface that connects to the Linera network via GraphQL to provide a real-time user experience.

## 🛠️ Setup & Installation

### Prerequisites

*   Rust and Cargo
*   Linera SDK and `linera` toolchain
*   Node.js and npm/yarn
*   Docker (optional, for running the Rust service)

### 1. Linera Application

```bash
cd linera-app

# Build the application
linera project build

# Start a local Linera network (in a separate terminal)
linera net up

# Publish the application
linera publish
# Note the published application ID, you'll need it for the frontend and orchestrator.
```

### 2. Rust Arena Orchestrator
```bash
cd rust-orchestrator

# Configure the service by setting environment variables
cp .env.example .env
# Edit .env with your Linera application ID, private key for the AI agent, and data feed API keys.

# Run the service
cargo run
# Or run with Docker:
docker build -t aetherarena-orchestrator .
docker run --env-file .env aetherarena-orchestrator
```

### 3. React Frontend
```bash
cd frontend

# Install dependencies
npm install

# Configure the application
# Create a .env.local file and set:
# VITE_LINERA_GRAPHQL_URL="http://localhost:8080/graphql"
# VITE_APPLICATION_ID="your-application-id-here"

# Start the development server
npm run dev
```
Open [http://localhost:5173](http://localhost:5173) (or the port indicated) to view the application.

## 🎮 How to Use
1. **Start Everything**: Ensure the local Linera net, Rust Orchestrator, and frontend are all running.
2. **Fund Your Wallet**: Use the Linera wallet in the frontend to request funds from the local faucet.
3. **Join an Arena**: The frontend will display active Arenas. Click on one to view the live odds and prediction feed.
4. **Place a Prediction**: Select an outcome and an amount. Your transaction will be confirmed near-instantly.
5. **Watch the AI**: Observe the "AI Champions" placing their own predictions and moving the odds in real-time!
6. **Resolution**: When the event concludes, the Rust Orchestrator will trigger the resolution, and winnings will be automatically distributed.

## 🔗 Linera Integration
This project leverages key features of the Linera stack:
- **Microchains**: Each Arena is a microchain, preventing congestion. Each user and AI agent has their own chain for personal state.
- **Cross-Chain Messages**: Predictions are implemented as cross-chain messages from a user's microchain to an Arena's microchain.
- **GraphQL API**: The frontend uses subscriptions to receive real-time push updates on market state, creating a fluid UX.
- **Real-Time Finality**: The core gameplay loop of quick predictions and resolutions relies on Linera's instant finality.

## 👥 Team
- [Gaurav Karakoti]
- Contact: [Your Telegram & X/Twitter Handle]

## 📜 Changelog
- Wave 2: Project ideation, basic Linera application skeleton, and Rust service structure.
- Wave 3: Implemented core Arena lifecycle, basic frontend, and AI agent integration for odds calculation.
- Wave 4 : Polished UX, added multiple AI champions, and implemented a robust resolution system.
