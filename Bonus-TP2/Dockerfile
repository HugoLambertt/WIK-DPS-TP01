FROM ubuntu:latest AS builder
RUN apt-get update && apt-get install -y nasm

WORKDIR /app
COPY main.asm .

RUN nasm -f bin main.asm -o mon_programme && chmod +x mon_programme

FROM scratch
COPY --from=builder /app/mon_programme /mon_programme
CMD ["/mon_programme"]