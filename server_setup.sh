#!/bin/bash

# Move .env.example to .env File
if [ -e .env.example ]; then
  echo ".env.example found"
  echo "rename .env.example to .env"
  mv .env.example .env
fi
# END Move .env.example to .env File

# Import .env File
if [ -f .env ]; then
  echo ".env found!"
  source .env
else
  echo "Please Set .env First!"
  exit 1
fi
# END Import .env File

# Docker Engine
if ! [ -f /etc/apt/keyrings/docker.gpg ]; then
  # Add Docker's official GPG key:
  sudo apt-get update
  sudo apt-get install ca-certificates curl gnupg
  sudo install -m 0755 -d /etc/apt/keyrings
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
  sudo chmod a+r /etc/apt/keyrings/docker.gpg

  # Add the repository to Apt sources:
  echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
    sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
  sudo apt-get update

  sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y
  sudo docker run hello-world

  # # Docker-Compose
  sudo curl -L "https://github.com/docker/compose/releases/download/v2.22.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
  sudo chmod +x /usr/local/bin/docker-compose
  docker-compose --version
  # END Docker-Compose
fi
# END Docker Engine

# Install Rust and sqlx-cli
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
cargo install sqlx-cli --no-default-features --features mysql
# END Install Rust and sqlx-cli

# Prepare Project
if [ -e migrations ]; then
  sudo docker-compose up -d
  sqlx migrate run
  cargo build --release
fi
# END Prepare Project

# TLS Server
if ! [ -f /etc/nginx/sites-available/actix ]; then
  sudo apt install nginx -y
  sudo cp ./miscellaneous/actix /etc/nginx/sites-available/
  sudo ln -s /etc/nginx/sites-available/actix /etc/nginx/sites-enabled/
  sudo nginx -t
  sudo systemctl restart nginx

  if ! [ -f /usr/bin/certbot ]; then
    sudo apt install snapd -y
    sudo snap install core; sudo snap refresh core
    sudo apt remove certbot
    sudo snap install --classic certbot
    sudo ln -s /snap/bin/certbot /usr/bin/certbot
  fi

  sudo ufw enable
  sudo ufw allow 'OpenSSH'
  sudo ufw allow 'Nginx Full'
  sudo ufw status
  sudo ufw delete allow 'Nginx HTTP'

  sudo certbot --nginx -d api.prodigicrud.rifuki.codes -d www.api.prodigicrud.rifuki.codes
fi
# END TLS Server

