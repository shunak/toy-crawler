# Import OS image
FROM ubuntu:20.04
FROM nginx
# FROM gcr.io/google.com/cloudsdktool/google-cloud-cli:latest
# ENV PORT 8080
# ENV HOST 0.0.0.0
# EXPOSE 8080

# Set OS environment
ENV DEBIAN_FRONTEND=noninteractive

# ADD from local
ADD cron.d /etc/cron.d/
ADD secrets /

# RUN OS commads
RUN apt-get update && apt-get install -y tzdata && apt-get install -y wget
RUN ln -fs /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
RUN dpkg-reconfigure --frontend noninteractive tzdata
RUN apt-get install -y cron && wget https://storage.googleapis.com/testhosting202209/toy-crawler && chmod 744 /toy-crawler
RUN apt-get install -y curl && apt-get install -y tar && apt-get install -y python && apt-get install -y vim && apt-get install -y git
RUN curl -O https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-sdk-359.0.0-linux-x86_64.tar.gz && tar zxvf google-cloud-sdk-359.0.0-linux-x86_64.tar.gz
RUN ./google-cloud-sdk/install.sh \
  --usage-reporting false \
  --rc-path ~/.bashrc \
  --command-completion true \
  --path-update true
RUN mv /.vimrc /root && chmod 744 /exe.sh && chmod 0644 /etc/cron.d/*
RUN /google-cloud-sdk/bin/gcloud auth activate-service-account --key-file /secret.json #&& /google-cloud-sdk/bin/gcloud components update

# RUN container
CMD service cron start && tail -f /dev/null 
# CMD service cron start && tail -f /dev/null && service nginx start
