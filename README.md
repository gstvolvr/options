# Deep in the money covered calls

Buy-write screener for deep in the money covered call strategy. Idea is to find
positions, with expiration dates usually over a year in the future, that provide a good
balance of downward protection (15% to 30%) and expected return (premium + dividends).

I discuss the strategy in more detail [here](https://www.oliver.dev/posts/2025/05/options-screener-part-i.html)

Positions are loaded into a personal Google Sheet and are updated every 30 minutes during market hours (9:30AM-4:00PM ET, Monday-Friday).

## Data Sources & Calculation

The system pulls real-time data from the **Schwab API** and calculates the following metrics for each deep in-the-money covered call position:

**Stock Information:**
* Company name and industry sector
* Current stock price and market data

**Options Information:**
* Call strike price and expiration date
* Latest bid, mid-point, and ask prices
* Net position cost (stock price - option premium)

**Strategy Metrics:**
* **Insurance**: Downside protection percentage
* **Premium**: Income from selling the call option
* **Expected Returns**: Projected returns after capturing 1-4 dividend payments
* Dividend amount and ex-dividend dates

All data is processed using a Rust-based calculation engine for performance and deployed to Google Cloud Run for automated execution.

## Cloud Infrastructure

This project uses several Google Cloud services:
- **Cloud Run Jobs**: Serverless execution during market hours only
- **Cloud Scheduler**: Automated triggering every 30 minutes (9:30AM-4:00PM ET)
- **Artifact Registry**: Private Docker image storage
- **Secret Manager**: Secure API credential storage

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed setup instructions.

## Running with Docker

This project can be run in a Docker container using the provided Dockerfile and docker-compose.yml.

### Prerequisites

- Docker and Docker Compose installed on your system
- A `.env` file with the necessary environment variables (see below)
- Google API credentials (secrets-manager-key.json)

### Environment Variables

Create a `.env` file in the project root with the following variables:

```
GOOGLE_SHEETS_ID=your_google_sheets_id
GOOGLE_SHEETS_CLIENT_SECRET=/app/secrets-manager-key.json
DATA_PATH=/app/data
```

## Running with Docker Compose

1. Build and run the container:

```bash
docker-compose up --build
```

2. To run in the background:

```bash
docker-compose up -d
```

3. To view logs:

```bash
docker-compose logs -f
```

4. To stop the container:

```bash
docker-compose down
```

## Development Notes

Some infrastructure and deployment scripts in this project were generated with AI assistance from Claude. These files are marked with appropriate headers. The core trading logic and calculations remain human-authored.
