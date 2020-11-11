
migrate db:

rm repo.db src/schema.rs
diesel migration run


Test commands:


curl -v -X POST -d '{ "sk": "mykey", "identiifier": "identiifier2", "human_name" : "human_name" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/project_add
curl -v -X POST -d '{  "sk": "f8f1208d-bf03-4daf-b919-ab18c20138b0" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/project_add


curl -v -X POST -d '{  "key": "HOME", "value" : "/home/username" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/keyvalue_add



curl -v -X POST -d '{  "key_value": [{  "key": "HOME", "value" : "/home/username" }] }' -H 'Content-Type: application/json' http://127.0.0.1:8888/enviroment_add


curl -v -X POST -d '{ "client_identifier" : "1"  }' -H 'Content-Type: application/json' http://127.0.0.1:8888/run_add

curl -v -X POST -d '{ "name" : "name" , "classname" : "classname", "time" : 1, "error_type" : "error_type", "error_message" : "error_message", "error_description" : "error_description", "system_out": "system_out", "system_err" :"system_err" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/test_case_error_add


curl -v -X POST -d '{ "name" : "name" , "classname" : "classname", "time" : 1, "failure_type" : "edddddrror_type", "failure_message" : "failure_message", "failure_description" : "failure_description", "system_out": "system_out", "system_err" :"system_err" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/test_case_failure_add