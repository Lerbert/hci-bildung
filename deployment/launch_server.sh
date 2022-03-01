#! /bin/bash

# Rename environment variables to match rockets expectations
export ROCKET_DATABASES="{hci_bildung={url=\"${DATABASE_URL}\"}}"
export ROCKET_PORT=${PORT:-8000}

# Launch rocket
./hci-bildung
