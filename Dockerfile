FROM rust:1.36-buster
RUN apt update && apt install -y ffmpegthumbnailer
WORKDIR /usr/src/hermit
COPY . .
RUN cargo install --path .
EXPOSE 8088
RUN useradd -ms /bin/bash app_user
USER app_user
WORKDIR /home/app_user
COPY ./static ./static
COPY ./templates ./templates
RUN mkdir thumbnails
RUN ls -alh
CMD ["hermit-rust"]
