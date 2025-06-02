# This script requires the google-cloud-firestore package
# Install it using: pip install google-cloud-firestore
# Or install all dependencies with: pip install -r requirements.txt
#
# To run this script:
# 1. Make sure you have set up Google Cloud credentials:
#    export GOOGLE_APPLICATION_CREDENTIALS="/path/to/your/service-account-key.json"
# 2. Run the script: python -m options.load_to_firestore
#
# This script reads data from data/schwab_returns.jsonl and loads it into
# a Cloud Firestore instance under the project name "options"

from google.cloud import firestore
import json
import os
import logging
import time

# Constants
PROJECT_ID = "options-282500"
COLLECTION_NAME = "options_returns"
DATA_PATH = os.getenv('DATA_PATH', 'data')
JSONL_FILE = f"{DATA_PATH}/schwab_returns.jsonl"
BATCH_SIZE = 500  # Number of documents to write in a batch
MAX_RETRIES = 3
SLEEP_SECONDS = 0.5  # Sleep time between batches

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

def convert_firestore_value(value_dict):
    """Convert Firestore value dictionary to Python value"""
    if "doubleValue" in value_dict:
        return value_dict["doubleValue"]
    elif "stringValue" in value_dict:
        return value_dict["stringValue"]
    elif "timestampValue" in value_dict:
        return value_dict["timestampValue"]
    elif "nullValue" in value_dict:
        return None
    else:
        return None

def extract_document_id(name):
    """Extract document ID from Firestore document name"""
    # Format: projects/options/databases/(default)/documents/options_returns/{document_id}
    return name.split('/')[-1]

def load_to_firestore():
    """Load data from JSONL file to Firestore"""
    # Initialize Firestore client
    db = firestore.Client(project=PROJECT_ID, database="options")

    # Create a batch
    batch = db.batch()
    count = 0
    total_count = 0

    logging.info(f"Starting to load data from {JSONL_FILE} to Firestore")

    try:
        with open(JSONL_FILE, 'r') as file:
            for line in file:
                try:
                    # Parse JSON line
                    doc_data = json.loads(line.strip())

                    # Extract document ID from name
                    doc_id = extract_document_id(doc_data["name"])

                    # Convert Firestore formatted fields to Python dict
                    fields = {}
                    for field_name, field_value in doc_data["fields"].items():
                        fields[field_name] = convert_firestore_value(field_value)

                    # Add document to batch
                    doc_ref = db.collection(COLLECTION_NAME).document(doc_id)
                    batch.set(doc_ref, fields)

                    count += 1
                    total_count += 1

                    # Commit batch when it reaches BATCH_SIZE
                    if count >= BATCH_SIZE:
                        retry_count = 0
                        while retry_count < MAX_RETRIES:
                            try:
                                batch.commit()
                                logging.info(f"Committed batch of {count} documents. Total: {total_count}")
                                batch = db.batch()
                                count = 0
                                time.sleep(SLEEP_SECONDS)
                                break
                            except Exception as e:
                                retry_count += 1
                                logging.error(f"Error committing batch (attempt {retry_count}): {e}")
                                if retry_count >= MAX_RETRIES:
                                    raise
                                time.sleep(SLEEP_SECONDS * 2)

                except json.JSONDecodeError as e:
                    logging.error(f"Error parsing JSON: {e}")
                    continue
                except Exception as e:
                    logging.error(f"Error processing document: {e}")
                    continue

        # Commit any remaining documents
        if count > 0:
            retry_count = 0
            while retry_count < MAX_RETRIES:
                try:
                    batch.commit()
                    logging.info(f"Committed final batch of {count} documents. Total: {total_count}")
                    break
                except Exception as e:
                    retry_count += 1
                    logging.error(f"Error committing final batch (attempt {retry_count}): {e}")
                    if retry_count >= MAX_RETRIES:
                        raise
                    time.sleep(SLEEP_SECONDS * 2)

        logging.info(f"Successfully loaded {total_count} documents to Firestore")

    except Exception as e:
        logging.error(f"Error loading data to Firestore: {e}")
        raise

if __name__ == '__main__':
    load_to_firestore()
