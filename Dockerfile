FROM python:3.9-slim-bullseye

# Install required system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set up working directory
WORKDIR /app

# Create rust binary directory (will be mounted or built separately)
RUN mkdir -p options-rs/target/release

# Copy Python requirements and install dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the application
COPY bin/ bin/
COPY options/ options/

# Create data directory (will be mounted as volume)
RUN mkdir -p data

# Create directory for virtual environment (not actually used in Docker, but referenced in script)
RUN mkdir -p .venv/bin
RUN touch .venv/bin/activate

# Set environment variables
ENV PYTHONPATH=/app
ENV DATA_PATH=/app/data

# Create a wrapper script that will handle environment variables
RUN echo '#!/bin/bash\n\
set -e\n\
\n\
# If .env file is provided as a volume, load it\n\
if [ -f /app/.env ]; then\n\
  export $(cat /app/.env | xargs)\n\
fi\n\
\n\
# Run the daily script\n\
exec /app/bin/daily.sh\n\
' > /app/docker-entrypoint.sh && chmod +x /app/docker-entrypoint.sh

# Set the entrypoint
ENTRYPOINT ["/app/docker-entrypoint.sh"]