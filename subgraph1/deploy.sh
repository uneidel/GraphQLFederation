#!/bin/bash

cargo  lambda build --release --arm64
#does not work as expected moving to self zipping and aws 
#cargo lambda deploy --retry-attempts 5 --region us-east-1 --binary-path ../target/lambda/user/bootstrap \
# --binary-name bootstrap --iam-role arn:aws:iam::xxxx:role/cargo-lambda-role subgraph1  --enable-function-url

if test -f "../deployment/temp/subgraph1.zip"; then
 echo "Deleting file"
 rm ../deployment/temp/subgraph1.zip
fi
zip -j ../deployment/temp/subgraph1.zip ../target/lambda/user/bootstrap
zip -j ../deployment/temp/subgraph1.zip ./config/Default.toml
# add more for additional layers

# deploy to aws
#TODO add error handling if function not exists
RESULT=$(aws lambda get-function --function-name subgraph1 | jq -r .Configuration.FunctionName) 

echo $RESULT

if [ "$RESULT" = "subgraph1" ]; then 
 echo "Updating"
 aws lambda update-function-code --function-name subgraph1 \
  --zip-file fileb://../deployment/temp/subgraph1.zip
else
    # Create New Function
    aws lambda create-function --function-name subgraph1 \
    --runtime provided.al2 \
    --role arn:aws:iam::502810299864:role/cargo-lambda-role \
    --zip-file fileb://../deployment/temp/subgraph1.zip \
    --description "subgraph1" \
    --timeout 5 \
    --handler UserFunction  \
    
fi

echo "Please add Public access to Function url if needed."



