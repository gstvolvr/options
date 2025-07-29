#!/bin/bash

# Script to update the Docker image for an existing deployment
# Usage: ./update_image.sh

set -e

PROJECT_ID=$(gcloud config get-value project)
REGION="us-east1"
REPOSITORY_NAME="options-repo"
IMAGE_NAME="options-rs"
JOB_NAME="options-processor"

if [ -z "$PROJECT_ID" ]; then
    echo "Error: No project ID found. Run 'gcloud config set project YOUR_PROJECT_ID'"
    exit 1
fi

echo "🔄 Updating Options Processor Image"
echo "Project: $PROJECT_ID"
echo ""

# Build and push new image
echo "🏗️  Building new Docker image..."
docker build -t $REGION-docker.pkg.dev/$PROJECT_ID/$REPOSITORY_NAME/$IMAGE_NAME:latest .

echo "📤 Pushing updated image..."
docker push $REGION-docker.pkg.dev/$PROJECT_ID/$REPOSITORY_NAME/$IMAGE_NAME:latest

# Update Cloud Run Job
echo "☁️  Updating Cloud Run Job..."
gcloud run jobs replace - <<EOF
apiVersion: run.googleapis.com/v1
kind: Job
metadata:
  name: $JOB_NAME
  annotations:
    run.googleapis.com/launch-stage: BETA
spec:
  template:
    spec:
      template:
        spec:
          containers:
          - image: $REGION-docker.pkg.dev/$PROJECT_ID/$REPOSITORY_NAME/$IMAGE_NAME:latest
            env:
            - name: RUST_LOG
              value: info
            - name: GOOGLE_CLOUD_PROJECT
              value: $PROJECT_ID
            resources:
              limits:
                cpu: "1"
                memory: "2Gi"
          restartPolicy: OnFailure
          timeoutSeconds: 1800
      parallelism: 1
      completions: 1
EOF

echo ""
echo "✅ Image updated successfully!"
echo ""
echo "🧪 Test the updated job:"
echo "  ./bin/test-job.sh"