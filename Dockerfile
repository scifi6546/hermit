FROM rust:1.36-buster
RUN apt update && apt install -y ffmpegthumbnailer npm
WORKDIR /usr/src/hermit
COPY . .
WORKDIR /usr/src/hermit/react-app
RUN npm install
RUN npm run deploy
WORKDIR /usr/src/hermit
RUN cargo install --path .
EXPOSE 8088
EXPOSE 8443
RUN useradd -ms /bin/bash app_user
USER app_user
WORKDIR /home/app_user
COPY ./static ./static
COPY ./templates ./templates
RUN mkdir thumbnails
RUN ls -alh
ENV SSL=""
CMD hermit-rust $SSL
