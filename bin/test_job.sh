#!/bin/bash

# Test script to manually trigger the options processor job
# Usage: ./test_job.sh

set -e

PROJECT_ID=$(gcloud config get-value project)
REGION="us-east1"
JOB_NAME="options-processor"

if [ -z "$PROJECT_ID" ]; then
    echo "Error: No project ID found. Run 'gcloud config set project YOUR_PROJECT_ID'"
    exit 1
fi

echo "🧪 Testing Options Processor Job"
echo "Project: $PROJECT_ID"
echo "Job: $JOB_NAME"
echo ""

echo "▶️  Executing job..."
gcloud run jobs execute $JOB_NAME --region=$REGION --wait

echo ""
echo "📊 View execution details:"
echo "https://console.cloud.google.com/run/jobs/details/$REGION/$JOB_NAME"