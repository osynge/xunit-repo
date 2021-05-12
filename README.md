# xunit-repo

## Introduction

This server stores junit/[xunit2](https://xunit.net/) style files in a local database. The [xunit-repo-client](https://github.com/osynge/xunit-repo-client) can upload [xunit2](https://xunit.net/) files to this server. The [xunit-repo-viewer](https://github.com/osynge/xunit-repo-viewer) project presents the database as a web site.

xunit-repo can be configured with environment variables, configuration files, or command line arguments, in order from lowest to highest precedence. xunit-repo is expected to be used either on the developers desktop, or as part of a continuous integration, continuous deployment framework such as jenkins or drone.

## To do:

* Documentation
    * Building the project.
    * configuration file.
    * command line argument.
    * environment variables.

### Table of xunit-repo-client configuration.

Setting | Type | Environment variable | Configuration parameter | Command line argument
------- | ---- | -------------------- | ----------------------- | ---------------------
Database URL | String | XR_DATABASE | database_url | --database-url
Migrate database | Boolean | XR_DATABASE_MIGRATE | database_migrate | --database-migrate --no-database-migrate
Host | String | XR_HOST | host | --host
Port | Integer | XR_PORT | port | --port
Configuration file | String | XR_CONFIG || --config
Log level| Integer | XR_LOG_JSON | loglevel | -v --verbose -q --quiet
Log in json | Boolean | XR_LOG_JSON | json_logs | --json-logging --line-logging
Viewer URL| String | XR_VIEWER_URL | viewer_url | --viewer-url

### Test commands:

curl -v -X POST -d '{ "sk": "mykey", "identiifier": "identiifier2", "human_name" : "human_name" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/project_add
curl -v -X POST -d '{  "sk": "f8f1208d-bf03-4daf-b919-ab18c20138b0" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/project_add


curl -v -X POST -d '{  "key": "HOME", "value" : "/home/username" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/keyvalue_add



curl -v -X POST -d '{  "key_value": [{  "key": "HOME", "value" : "/home/username" }] }' -H 'Content-Type: application/json' http://127.0.0.1:8888/enviroment_add


curl -v -X POST -d '{ "client_identifier" : "1"  }' -H 'Content-Type: application/json' http://127.0.0.1:8888/run_add

curl -v -X POST -d '{ "name" : "name" , "classname" : "classname", "time" : 1, "error_type" : "error_type", "error_message" : "error_message", "error_description" : "error_description", "system_out": "system_out", "system_err" :"system_err" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/test_case_error_add


curl -v -X POST -d '{ "name" : "name" , "classname" : "classname", "time" : 1, "failure_type" : "edddddrror_type", "failure_message" : "failure_message", "failure_description" : "failure_description", "system_out": "system_out", "system_err" :"system_err" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/test_case_failure_add

curl -v -X POST -d '{ "name" : "name" , "classname" : "classname", "time" : 1, "skipped_message" : "edddddrror_type" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/test_case_skipped_add

curl -v -X POST -d '{ "name" : "namedddasas" , "classname" : "classnameddddassas", "time" : 1  }' -H 'Content-Type: application/json' http://127.0.0.1:8888/test_case_pass_add
