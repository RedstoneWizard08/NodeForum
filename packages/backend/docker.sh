#!/bin/bash

docker run -it --rm --name nodeforum_postgres --network host -e POSTGRES_HOST_AUTH_METHOD=trust postgresql
docker run -it --rm --name nodeforum_pgadmin --network host dpage/pgadmin4:latest
