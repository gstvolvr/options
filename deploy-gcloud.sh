#!/bin/bash

# Google Cloud Deployment Script for Options Processor
# This script sets up Cloud Run Job + Cloud Scheduler for market hours processing

set -e

# Configuration
PROJECT_ID=$(gcloud config get-value project)
REGION="us-east1"
REPOSITORY_NAME="options-repo"
IMAGE_NAME="options-rs"
JOB_NAME="options-processor"
SCHEDULER_NAME="options-schedule"
SERVICE_ACCOUNT_NAME="options-scheduler"

if [ -z "$PROJECT_ID" ]; then
    echo "Error: No project ID found. Run 'gcloud config set project YOUR_PROJECT_ID'"
    exit 1
fi

echo "🚀 Deploying Options Processor to Google Cloud"
echo "Project: $PROJECT_ID"
echo "Region: $REGION"
echo ""

# Step 1: Create Artifact Registry repository
echo "📦 Setting up Artifact Registry..."
if ! gcloud artifacts repositories describe $REPOSITORY_NAME --location=$REGION &>/dev/null; then
    gcloud artifacts repositories create $REPOSITORY_NAME \
        --repository-format=docker \
        --location=$REGION \
        --description="Docker repository for options processor"
    echo "✅ Artifact Registry repository created"
else
    echo "ℹ️  Artifact Registry repository already exists"
fi

# Step 2: Configure Docker authentication
echo "🔑 Configuring Docker authentication..."
gcloud auth configure-docker $REGION-docker.pkg.dev --quiet

# Step 3: Build and push Docker image
echo "🏗️  Building Docker image..."
docker build -t $REGION-docker.pkg.dev/$PROJECT_ID/$REPOSITORY_NAME/$IMAGE_NAME:latest .

echo "📤 Pushing Docker image..."
docker push $REGION-docker.pkg.dev/$PROJECT_ID/$REPOSITORY_NAME/$IMAGE_NAME:latest
echo "✅ Docker image pushed successfully"

# Step 4: Create service account for scheduler
echo "👤 Setting up service account..."
if ! gcloud iam service-accounts describe $SERVICE_ACCOUNT_NAME@$PROJECT_ID.iam.gserviceaccount.com &>/dev/null; then
    gcloud iam service-accounts create $SERVICE_ACCOUNT_NAME \
        --display-name="Options Scheduler Service Account" \
        --description="Service account for triggering options processor jobs"
    echo "✅ Service account created"
else
    echo "ℹ️  Service account already exists"
fi

# Step 5: Create Cloud Run Job
echo "☁️  Creating Cloud Run Job..."
gcloud run jobs create $JOB_NAME \
    --image $REGION-docker.pkg.dev/$PROJECT_ID/$REPOSITORY_NAME/$IMAGE_NAME:latest \
    --region $REGION \
    --memory 2Gi \
    --cpu 1 \
    --task-timeout 1800 \
    --parallelism 1 \
    --set-env-vars RUST_LOG=info,GOOGLE_CLOUD_PROJECT=$PROJECT_ID \
    --max-retries 3

echo "✅ Cloud Run Job created"

# Step 6: Grant IAM permissions
echo "🔐 Setting up IAM permissions..."
gcloud run jobs add-iam-policy-binding $JOB_NAME \
    --member="serviceAccount:$SERVICE_ACCOUNT_NAME@$PROJECT_ID.iam.gserviceaccount.com" \
    --role="roles/run.invoker" \
    --region=$REGION

echo "✅ IAM permissions configured"

# Step 7: Create Cloud Scheduler job
echo "⏰ Setting up Cloud Scheduler..."
gcloud scheduler jobs delete $SCHEDULER_NAME --quiet &>/dev/null || true

gcloud scheduler jobs create http $SCHEDULER_NAME \
    --schedule="*/30 9-16 * * 1-5" \
    --time-zone="America/New_York" \
    --uri="https://$REGION-run.googleapis.com/apis/run.googleapis.com/v1/namespaces/$PROJECT_ID/jobs/$JOB_NAME:run" \
    --http-method=POST \
    --oauth-service-account-email=$SERVICE_ACCOUNT_NAME@$PROJECT_ID.iam.gserviceaccount.com \
    --headers="Content-Type=application/json" \
    --location=$REGION

echo "✅ Cloud Scheduler job created"

echo ""
echo "🎉 Deployment complete!"
echo ""
echo "📊 Summary:"
echo "  • Docker image: $REGION-docker.pkg.dev/$PROJECT_ID/$REPOSITORY_NAME/$IMAGE_NAME:latest"
echo "  • Cloud Run Job: $JOB_NAME"
echo "  • Schedule: Every 30 minutes, 9:30AM-4:00PM ET, Monday-Friday"
echo "  • Estimated cost: ~$0.50-2.00/month"
echo ""
echo "🔍 Monitor your deployment:"
echo "  • Cloud Run: https://console.cloud.google.com/run/jobs/details/$REGION/$JOB_NAME"
echo "  • Cloud Scheduler: https://console.cloud.google.com/cloudscheduler"
echo ""
echo "🧪 Test the job manually:"
echo "  gcloud run jobs execute $JOB_NAME --region=$REGION"