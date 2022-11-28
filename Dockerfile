FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y tzdata && apt-get install -y wget
RUN ln -fs /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
RUN dpkg-reconfigure --frontend noninteractive tzdata
RUN apt-get install -y cron && wget https://storage.googleapis.com/testhosting202209/toy-crawler && chmod 744 /toy-crawler 
RUN apt-get install -y curl && apt-get install -y tar && apt-get install -y python && apt-get install -y vim && apt-get install -y git
RUN curl -O https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-sdk-359.0.0-linux-x86_64.tar.gz
RUN tar zxvf google-cloud-sdk-359.0.0-linux-x86_64.tar.gz
RUN ./google-cloud-sdk/install.sh \
  --usage-reporting false \
  --rc-path ~/.bashrc \
  --command-completion true \
  --path-update true
ADD cron.d /etc/cron.d/
ADD secrets /
RUN mv /.vimrc /root && chmod 744 /exe.sh && gcloud auth activate-service-account --key-file secret.json
RUN chmod 0644 /etc/cron.d/*

CMD service cron start && tail -f /dev/null

# CMD ["nginx", "-g", "daemon off;"]

