FROM ubuntu:latest

WORKDIR /app

COPY ./target/release/benson /app/benson
COPY ./config.json ./config.json
RUN cp /app/config.json /config.json

RUN apt-get update -y
RUN apt-get install software-properties-common -y
RUN add-apt-repository universe
RUN apt-get update -y
RUN apt-get install -y ffmpeg
# RUN curl -L https://yt-dl.org/downloads/latest/youtube-dl -o /usr/local/bin/youtube-dl
# RUN chmod a+rx /usr/local/bin/youtube-dl

COPY ./docker-entrypoint.sh ./docker-entrypoint.sh

CMD ["/app/docker-entrypoint.sh"]