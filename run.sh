#!/bin/bash


StartSubgraph1 () {
    echo "subgraph1 9002"
  (
    cd manufacturer 
    cargo lambda watch --invoke-port 9002 > ../.testdata/subgraph1.log 2>&1 & 
    echo $! > ../.testdata/subgraph1.txt
  )
}
StartUser () {
    echo "USER 9001"
  (cd user 
   cargo lambda watch --invoke-port 9001 > ../.testdata/user.log 2>&1 &
   echo $! > ../.testdata/user.txt
 )
}


StartRover(){
  echo "Rover"
  rover dev --name subgraph1 --supergraph-port 1234 --url http://localhost:9002/lambda-url/subgraph1/graphql &
  rover dev --name user --supergraph-port 1234 --url http://localhost:9001/lambda-url/user/graphql &
  
}

StopUser () {
  FILE=./.testdata/user.txt
  if test -f "$FILE"; then
    kill -9 `cat ./.testdata/user.txt`
    rm -- "$FILE"
  fi
}
StopSubgraph1 () {
  FILE=./.testdata/subgraph1.txt
  if test -f "$FILE"; then
    kill -9 `cat ./.testdata/subgraph1.txt`
    rm -- "$FILE"
  fi
}

StopRover (){
  kill $(ps aux | grep 'rover dev' | awk '{print $2}')
}
FILE=./.testdata/user.txt
if test -f "$FILE"; then
    echo "stopping..."
    StopSubgraph1
    StopUser
    
else 
    echo "starting..."
    StartSubgraph1
    StartUser
    
    #StartRover
fi
