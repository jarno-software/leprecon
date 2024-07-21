#!/bin/bash

# Wait for the endpoint to return a 200 response
until curl --fail --silent --insecure --output /dev/null "https://127.0.0.1"; do
    sleep 5
    echo "Retrying"
done

echo "Endpoint is up!"