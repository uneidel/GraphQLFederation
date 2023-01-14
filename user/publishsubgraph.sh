#!/bin/bash

rover graph introspect http://localhost:9000/lambda-url/user/graphql > user.graphql

rover subgraph publish securrent@se \
  --schema "./user.graphql" \
  --name user \
  --routing-url "https://my-running-subgraph.com/api"