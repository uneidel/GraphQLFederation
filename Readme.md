# GraphQl on Lambda with Apollo Router 

Boilerplate Graphql Federation using rust using Async Graphql, Apollo Router, Keycloak

Main Features:
- JWT    
- Configuration
- config based routing   
- Full Local Development Environment including keycloak IDP
- Deployment Scripts and Templates for AWS Lambda, K8s Apollo router, keycloak




## Prerequisities
cargo install cargo-make
pip3 install cargo-lambda
curl -sSL https://rover.apollo.dev/nix/latest | sh
cargo make docker up -d keycloak

## Test Lambdas locally
Change to Directory ...
cargo lambda watch
eg. curl http://127.0.0.1:9000/lambda-url/sample/

### Makefile


### Rover
use cli for generating supergraph schema

## Deploy to AWS

**Note**: Do  not with Root account - create admin IAM role before
aws iam create-role --role-name rust-role --assume-role-policy-document file://rust-role.json



## Keycloak
admin/admin

http://localhost:8080/realms/silicon/.well-known/openid-configuration
http://localhost:8080/realms/silicon/protocol/openid-connect/certs

> Hint: 
clients > [Client] > Client Scopes > Setup > [Scope w/ description = "Dedicated scope and mappers for this client"]

### jwt Helper
jwtd() {
    if [[ -x $(command -v jq) ]]; then
         jq -R 'split(".") | .[0],.[1] | @base64d | fromjson' <<< "${1}"
         echo "Signature: $(echo "${1}" | awk -F'.' '{print $3}')"
    fi
}

## Compose Supergraph

Please see publishsubgraph.sh   



### Main creates
Tokio     
lambda_runtime   
async-graphql    
warp    