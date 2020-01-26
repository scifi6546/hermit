FROM rust:1.40-buster
RUN apt update && apt install -y ffmpegthumbnailer npm
WORKDIR /usr/src/hermit
COPY . .
WORKDIR /usr/src/hermit/react-client
run mkdir /usr/src/hermit/static
run echo "" >> public/index.html
RUN ls
RUN ls public/
RUN npm install
RUN npm run deploy
WORKDIR /usr/src/hermit
RUN ls -al .
RUN cargo install --path .
EXPOSE 8088
EXPOSE 8443
RUN useradd -ms /bin/bash app_user

RUN cp -r /usr/src/hermit/static /home/app_user/static
USER app_user
WORKDIR /home/app_user
RUN mkdir thumbnails
RUN ls -alh
ENV SSL=""
CMD hermit-rust $SSL
