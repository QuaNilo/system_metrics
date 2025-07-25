#! /bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if 'ab' is installed, install if missing
if ! command -v ab &> /dev/null; then
    echo "'ab' not found. Installing apache2-utils..."
    sudo apt update
    sudo apt install -y apache2-utils
else
    echo "'ab' is already installed."
fi

ab -n 1000 -c 50 http://0.0.0.0:50000/system/info
