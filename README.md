# Deep in the money covered calls

Buy-write screener for deep in the money covered call strategy. Idea is to find
positions, with expiration dates usually over a year in the future, that provide a good
balance of downward protection (15% to 30%) and expected return (premium + dividends).

I discuss the strategy in more detail [here](https://www.oliver.dev/posts/2025/05/options-screener-part-i.html)

Positions are loaded into
[this](https://docs.google.com/spreadsheets/d/1dhLDNkZbI2-7Fm4jXRreL-S6oTRExumvJRH1fEQIiOs/edit?usp=sharing) Google Sheet and are updated every 15 minutes on trading days.

# Data
I pull data from the Schwab API and track the following attributes:
* name	
* industry	
* stock price
* net price of position 
* call strike price
* call expiration date
* insurance	
* premium	
* dividend	
* ex dividend date	
* expected return after 1 dividend
* expected return after 2 dividends	
* expected return after 3 dividends	
* expected return after 4 dividends	
* latest bid	
* latest mid point	
* latest ask

# Running with Docker

This project can be run in a Docker container using the provided Dockerfile and docker-compose.yml.

## Prerequisites

- Docker and Docker Compose installed on your system
- A `.env` file with the necessary environment variables (see below)
- Google API credentials (secrets-manager-key.json)

## Environment Variables

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

## Running the Container Directly

If you prefer to run the Docker container directly without Docker Compose:

```bash
# Build the image
docker build -t options-screener .

# Run the container
docker run --rm \
  -v $(pwd)/.env:/app/.env:ro \
  -v $(pwd)/secrets-manager-key.json:/app/secrets-manager-key.json:ro \
  -v $(pwd)/data:/app/data \
  options-screener
```
