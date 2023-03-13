FROM alpine:3
RUN apk add --no-cache sudo build-base alpine-sdk bash direnv glab
RUN apk fix
RUN adduser -D packager
RUN addgroup packager abuild
RUN echo "packager ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
#USER packager
#WORKDIR /home/packager
