# Hermit
Hermit is a home video server that *stays* at home. Unlike plex and emby it does not 'phone home'. Hermit is designed to keep 
your data where it belongs at home.
## Features
Right now hermit is *very* limited. It can play videos and playlists and that is about it.
worked on more seriously once there is a solid core server. 

## Limitations
Hermit by default does not work over https. I highly recomend using a reverse proxy like nginx or varnish to add https.
Hermit has also not been audited so the security is probably not very good. Go at your own risk!
## Run Instructions
  To run this program on docker run 
    ```docker run -e HERMIT_CONFIG=/home/app/config.json scifi6546/hermit```
  to run this app as a python app first create a venv with
  ```python3 -m venv venv```
  Activate the venv
   ``` source ./venv/bin/activate ```
  Install dependicies
  ```  pip3 install -e pacakge/ ```
  Install ffmpegthumbnailer
    (on ubuntu)
    ```apt install ffmpegthumbnailer```
  run the app
  ```  gunicorn app:app -b 0.0.0.0:8080 --timeout "120" -e HERMIT_CONFIG=config.json ```
## License
Hermit is licensed under the gplv3

