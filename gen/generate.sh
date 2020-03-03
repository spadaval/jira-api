#!/bin/sh
PATH_TO_JAR=./openapi-generator-cli.jar 

java -jar ${PATH_TO_JAR} generate -i ./swagger-v3.v3.json -g rust -o .. -c ./config.yaml
