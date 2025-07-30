#!/bin/bash
USER="$PAM_USER"
IP="$PAM_RHOST"
DATE="$(date '+%Y-%m-%d %H:%M:%S')"

echo "User $PAM_USER logged in from $PAM_RHOST at $DATE" >> /var/log/ssh-login.log

# Call your Rust email sender executable with arguments
/usr/local/bin/send_email ssh_login "$USER" "$IP" "$DATE"