# README #

This README would normally document whatever steps are necessary to get your application up and running.

### What is this repository for? ###

* Quick summary

This project implements a simple and very basic `CEP` framework. It was created for my rust programming purposes.
The idea is to create predefined blocks which can be joined and create virtualized apps. Rest api allows creation of definitions and deployments.
Definitions are blueprints of pieces which will create deployments which will process the data. 
Application is connected to kafka topic, once it gets all messages for deployment input it will run a block and produce (if defined) a message as a product of deployment.


* Version

There is no release as for now

### How do I get set up? ###

* Summary of set up

    To develop it locally following stuff is needed:
    * rust and cargo
    * python
    * docker

To run development environment run `docker compose down --rmi local --remove-orphans && docker compose --file=docker-compose.yaml up` which will provide all required elements.

* Configuration

  Project has few configuration places
  * `dev.toml` - it has config options related to application itself. Env configs take precedence for example for `kafka.hosts` `KAFKA_HOSTS` env will be used first.
  * `Rocket.toml` - for rocket configuration

* Dependencies
  * `pyo3` is the most important dependend

* Database configuration
  Database is auto migrated, all db configuration is in `dev.toml` file.

* How to run tests `make test` - vast majority of code is untested
* Deployment instructions
  
  In order to run application all services from the docker compose must be running. Then definition and deployments can be created

### Concepts ###

There are two main concepts here:
  * definition - block definition which will serve as a template for runtime
  * deployment - set of connected block which materialize subset of blocks  

As for now application supports only python code blocks - all is required is to implement a python function named `logic`
which takes as argument a dictionary and returns a dictionary

### Who do I talk to? ###

* Repo owner or admin
* Other community or team contact

### Dependencies ###
Install dependency like `python3.11 -m pip install psycopg2-binary` in order to use `psycopg2` in the scripts.

### Tools ###
  * [tools](https://github.com/rwadowski/mcep-tools) - tools repository - some scripts helping with testing,
  * [scripts](https://github.com/rwadowski/mcep-scripts) - python code block sources

### Example ### 
 1. Creation of block - `/definition` endpoint
    ```json
    {
    "name": "name",
    "version": "1.0",
    "body": {
        "type": "Github",
        "inputs": [
            {
                "name": "x",
                "data_type": "Text"
            },
            {
                "name": "y",
                "data_type": "Text"
            }
        ],
        "outputs": [
            {
                "name": "z",
                "data_type": "Text"
            }
        ],
        "source": {
            "owner": "rwadowski",
            "repository": "mcep-scripts",
            "token": "{{GITHUB_TOKEN}}",
            "path": "sum.py"
        }
      }
    } 
    ```
 2. Create a deployment - `/deployment` endpoint
    ```json
    {
    "name": "deployment",
    "version": "1.0.0",
    "sources": [
        {
            "id": "mcep-kafka-source", // see config in `dev.toml`
            "data_type": "Text"
        }
    ],
    "sinks": [
        {
            "id": "mcep-kafka-sink", // see config in `dev.toml`
            "data_type": "Text"
        }
    ],
    "blocks": [
        {
            "definition_id": 1,
            "id": 1
        }
    ],
    "connections": [
        {
            "from": {
                "source": "mcep-kafka-source", // see config in `dev.toml`
                "data_type": "Text"
            },
            "to": {
                "block": "1.1",
                "data_type": "Text"
            }
        },
        {
            "from": {
                "block": "1.1",
                "data_type": "Text"
            },
            "to": {
                "sink": "mcep-kafka-sink", // see config in `dev.toml`
                "data_type": "Text"
            }
        }
      ]
    }
    ```
 3. Send a message formatted like 
    ```json
    {
      "name": "x", //name of input in the definition
      "value": {
        "Text": "Hello", //value of type Text = "Hello"
      },
      "ts": 0 //timestamp millis
    }
    ```
 4. Send similar message for second name in definition above (`y`)
 5. If both inputs will be in place - a new message should produce a message to `output` topic with name = `z`