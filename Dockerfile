FROM	debian:latest
MAINTAINER Vijo Cherian codervijo@gmail.com

RUN	apt-get -y update && apt-get install -y curl
RUN	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >/root/rustup.sh
RUN	apt-get -y install procps net-tools ssh
RUN	apt-get -y install openssl
RUN	apt-get -y install libssl-dev pkg-config
RUN	apt-get -y install build-essential
RUN apt-get -y install libxml2 libxml2-dev cmake libcurl4-openssl-dev
RUN apt-get -y install protobuf-compiler libprotobuf-dev
RUN apt-get -y install net-tools vim file

RUN apt-get install -y libpcap-dev net-tools
RUN apt-get install -y gnuplot graphviz

RUN /bin/bash -x /root/rustup.sh -y

RUN apt-get -y install curl wget


WORKDIR /usr/src/app
#CMD		["lua", "/usr/src/luascope/scope"]
