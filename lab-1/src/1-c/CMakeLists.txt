cmake_minimum_required(VERSION 3.16)

project(networks-lab-1-c)

set(CMAKE_C_COMPILER clang)
set(CMAKE_C_FLAGS -std=c11)

set (CLIENT_SOURCES
    client.c
)

set(SERVER_SOURCES
    server.c
)

set(DEPS
    # -l
)

add_executable(client-c ${CLIENT_SOURCES})
add_executable(server-c ${SERVER_SOURCES})