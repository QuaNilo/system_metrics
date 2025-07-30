# Setup automatic SSH email notifications
### To setup the scripts where you get notified when a user logs in to server using ssh do the following:

- Add **ssh-login.sh** to **/usr/local/bin/**

- Make it executable ``sudo chmod +x /usr/local/bin/ssh-login.sh``

- Open **/etc/pam.d/sshd** and add near the end ``session optional pam_exec.so /usr/local/bin/ssh-login-hook.sh``

- Restart SSH ``` sudo systemctl restart sshd ```