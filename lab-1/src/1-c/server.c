#include <stdio.h>
#include <unistd.h>

#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>

int main(int argc, char **argv) {
    struct sockaddr_in sock_addr;
    sock_addr.sin_family = AF_INET;
    sock_addr.sin_port = htons(8080);
    sock_addr.sin_addr.s_addr = htonl(INADDR_ANY);

    int socket_fd = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (socket_fd < 0) {
        fprintf(stderr, "Couldn't create socket!");
        return -1;
    }
    {
        int enable = 1;
        if (setsockopt(socket_fd, SOL_SOCKET, SO_REUSEADDR, &enable, sizeof(int)) < 0) {
            fprintf(stderr, "Couldn't SO_REUSEADDR!");

            close(socket_fd);
            return -1;
        }
    }

    if (bind(socket_fd, (struct sock_addr*)&sock_addr, sizeof(sock_addr)) != 0) {
        close(socket_fd);

        fprintf(stderr, "Couldn't bind()!");
        return -1;
    }

    if (listen(socket_fd, SOMAXCONN) != 0) {
        fprintf(stderr, "Couldn't listen()!");

        close(socket_fd);
        return -1;
    }

    struct sockaddr_in from;
    int from_len = 0;
    int client_socket_fd = -1;
    for(;;) {
        client_socket_fd = -1;
        client_socket_fd = accept(socket_fd, (struct sock_addr*)&from, &from_len);
        if (client_socket_fd < 0) {
            fprintf(stderr, "Couldn't accept()!");

            close(socket_fd);
            return -1;
        }

        char buff[] = "Hello, world!";
        if (send(client_socket_fd, buff, sizeof(buff), MSG_NOSIGNAL) < 0) {
            fprintf(stderr, "Couldn't send()!");

            close(client_socket_fd);
            close(socket_fd);
            return -1;
        }

        close(client_socket_fd);
    }

    close(socket_fd);

    return 0;
}