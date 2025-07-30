# Google Cloud Deployment Guide

<!-- This guide was generated with AI assistance from Claude -->

This guide covers deploying the Options Processor as a Cloud Run Job with Cloud Scheduler for automated market hours execution.

## Prerequisites

1. **Google Cloud SDK** installed and authenticated
2. **Docker** installed and running
3. **Google Cloud Project** with billing enabled
4. **Required APIs** enabled:
   - Cloud Run API
   - Cloud Scheduler API
   - Artifact Registry API
   - IAM API

## Quick Start

### 1. Enable Required APIs
```bash
gcloud services enable run.googleapis.com
gcloud services enable cloudscheduler.googleapis.com
gcloud services enable artifactregistry.googleapis.com
gcloud services enable iam.googleapis.com
```

### 2. Set Your Project
```bash
gcloud config set project YOUR_PROJECT_ID
```

### 3. Deploy
```bash
./deploy-gcloud.sh
```

That's it! The script will handle everything automatically.

## Architecture

- **Cloud Run Job**: Executes your Rust application on-demand
- **Cloud Scheduler**: Triggers every 30 minutes during market hours (9:30AM-4:00PM ET, Mon-Fri)
- **Artifact Registry**: Stores Docker images
- **Secret Manager**: Stores API credentials (already configured)

## Manual Operations

### Test the Job
```bash
./scripts/test_job.sh
```

### Update Code
```bash
# Make your code changes, then:
./scripts/update_image.sh
```

### Monitor Executions
- **Cloud Run Console**: https://console.cloud.google.com/run
- **Cloud Scheduler Console**: https://console.cloud.google.com/cloudscheduler
- **Logs**: `gcloud logging read "resource.type=cloud_run_job"`

## Schedule Details

**Cron Expression**: `*/30 9-16 * * 1-5`
- Every 30 minutes
- Between 9:00 AM and 4:00 PM
- Monday through Friday
- Eastern Time (automatically handles DST)

**Market Hours Coverage**:
- Pre-market: Not covered (intentional)
- Regular hours: 9:30 AM - 4:00 PM ET ✅
- After-hours: Not covered (intentional)

## Troubleshooting

### Job Fails to Start
1. Check service account permissions
2. Verify image exists in Artifact Registry
3. Check Cloud Run Job configuration

### Scheduling Issues
1. Verify Cloud Scheduler job is enabled
2. Check service account has `roles/run.invoker` permission
3. Confirm timezone settings

### Authentication Errors
1. Ensure Secret Manager secrets exist
2. Verify service account has Secret Manager access
3. Check Google Cloud credentials

## Security

- **Service Account**: Minimal permissions (Cloud Run invoker only)
- **Secrets**: Stored in Secret Manager, not in container
- **Network**: Cloud Run jobs run in Google's secure environment
- **Images**: Stored in private Artifact Registry

## Scaling

Current configuration:
- **CPU**: 1 vCPU
- **Memory**: 2 GiB
- **Timeout**: 30 minutes
- **Parallelism**: 1 (single execution)

To adjust resources, modify the deployment script or use:
```bash
gcloud run jobs update options-processor \
  --memory 4Gi \
  --cpu 2 \
  --region us-east1
```

## Cleanup

To remove all resources:
```bash
# Delete scheduler
gcloud scheduler jobs delete options-schedule --location us-east1

# Delete Cloud Run job
gcloud run jobs delete options-processor --region us-east1

# Delete service account
gcloud iam service-accounts delete options-scheduler@PROJECT_ID.iam.gserviceaccount.com

# Delete Artifact Registry (optional)
gcloud artifacts repositories delete options-repo --location us-east1
```