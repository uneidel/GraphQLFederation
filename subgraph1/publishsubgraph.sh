#!/bin/bash

rover graph introspect http://localhost:9000/lambda-url/subgraph1/graphql > subgraph1.graphql
rover subgraph publish define@define \
  --schema "./subgraph1.graphql" \
  --name pcn \
  --routing-url "https://my-running-subgraph.com/api"