#!/bin/bash

## Format: project-service-env
#gcloud iam service-accounts create options-firestore-prod \
#    --display-name="Options Firestore Production Service Account" \
#    --description="Service account for Firestore operations in production"
#
## Replace YOUR_PROJECT_ID with your actual project ID
#gcloud projects add-iam-policy-binding options-282500 \
##    --member="serviceAccount:options-firestore-prod@options-282500.iam.gserviceaccount.com" \
#    --role="roles/datastore.user"
#gcloud iam service-accounts keys create firestore-key.json \
#    --iam-account=options-firestore-prod@options-282500.iam.gserviceaccount.com

# 1. Create a new service account (replace 'secrets-manager-sa' with your preferred name)
gcloud iam service-accounts create secrets-manager-sa \
    --display-name="Secret Manager Service Account"

# 2. Get your project ID (if you haven't set it)
PROJECT_ID=$(gcloud config get-value project)

# 3. Assign Secret Manager roles to the service account
# This grants access to view and manage secrets
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:secrets-manager-sa@$PROJECT_ID.iam.gserviceaccount.com" \
    --role="roles/secretmanager.secretAccessor"

# For managing secrets (create/delete/update), add admin role
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:secrets-manager-sa@$PROJECT_ID.iam.gserviceaccount.com" \
    --role="roles/secretmanager.admin"

# 4. Create and download the JSON key file (store securely!)
gcloud iam service-accounts keys create secrets-manager-key.json \
    --iam-account=secrets-manager-sa@$PROJECT_ID.iam.gserviceaccount.com