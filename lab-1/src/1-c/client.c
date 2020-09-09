#include <stdio.h>

#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>

int main(int argc, char** argv) {
    struct sockaddr_in sock_addr;
    sock_addr.sin_family = AF_INET;
    sock_addr.sin_port = htons(8080);
    inet_pton(AF_INET, "127.0.0.1", &(sock_addr.sin_addr));

    int socket_fd = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (socket_fd < 0) {
        fprintf(stderr, "Couldn't create socket!");
        return -1;
    }

    if (connect(socket_fd, (struct sockaddr*)&sock_addr, sizeof(sock_addr)) != 0) {
        fprintf(stderr, "Couldn't connect()!");

        close(socket_fd);
        return -1;
    }

    char buff[14];
    if (recv(socket_fd, buff, sizeof(buff), MSG_NOSIGNAL) < 0) {
        fprintf (stderr, "Couldn't recv()!");

        close(socket_fd);
        return -1;
    }

    shutdown(socket_fd, SHUT_RDWR);

    printf(buff);

    close(socket_fd);

    return 0;
}
