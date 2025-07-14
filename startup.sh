#!/usr/bin/zsh

if [[ -f "docker-compose.yml" ]]; then
  docker compose --profile full up -d
else
  echo "docker-compose.yml NOT found in the current directory."
fi
