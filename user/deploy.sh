#!/bin/bash

cargo  lambda build --release --arm64
#does not work as expected moving to self zipping and aws 
#cargo lambda deploy --retry-attempts 5 --region us-east-1 --binary-path ../target/lambda/user/bootstrap \
# --binary-name bootstrap --iam-role arn:aws:iam::502810299864:role/cargo-lambda-role UserFunction  --enable-function-url

if test -f "../deployment/temp/userfunction.zip"; then
 echo "Deleting file"
 rm ../deployment/temp/userfunction.zip
fi
zip -j ../deployment/temp/userfunction.zip ../target/lambda/user/bootstrap
zip -j ../deployment/temp/userfunction.zip ./config/Default.toml
# add more for additional layers

# deploy to aws
#TODO add error handling if function not exists
RESULT=$(aws lambda get-function --function-name UserFunction | jq -r .Configuration.FunctionName) 

echo $RESULT

if [ "$RESULT" = "UserFunction" ]; then 
 echo "Updating"
 aws lambda update-function-code --function-name UserFunction \
  --zip-file fileb://../deployment/temp/userfunction.zip
else
    # Create New Function
    aws lambda create-function --function-name UserFunction \
    --runtime provided.al2 \
    --role arn:aws:iam::502810299864:role/cargo-lambda-role \
    --zip-file fileb://../deployment/temp/userfunction.zip \
    --description "UserFunction" \
    --timeout 5 \
    --handler UserFunction  \
    
fi

echo "Please add Public access to Function url if needed."



