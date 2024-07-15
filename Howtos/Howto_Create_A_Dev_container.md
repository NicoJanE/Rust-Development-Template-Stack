---
layout: default
---

_This file is part of: PHP Development Template Stack_
_Copyright (c) 2024 Nico Jan Eelhart_

_This source code is licensed under the MIT License found in the  'LICENSE.md' file in the root directory of this source tree._
<br><br>

# 1 Create and start the container

<br>
To start the rust basic container in Docker Desktop execute this command from the **this**  directory:  

```
docker-compose -f compose_rust_cont.yml up -d --build --force-recreate
```
<br>

### 1.1 Optional add a Web Rocket project (using a add-on Docker compose file)
Optional, you can add one or more ***'Rocket'***  web framework projects, this creates a new project (directory) and installs the needed libraries 
1. For this, first you must set an environment variable which indicates the project name (which will be a directory). In case of a Windows host, set an environment variable with:
```
$env:PRJ_NAME_ARG="name_of_project_directory"
```
> *Note:*{: style="color: orange;font-size:13px; "} <br>
> <small>When you don't set this variable a ***default*** will be used, taken from the file: ***.env*** ('default_project_name')</small>

2. Then executing the add-on compose file
```
docker-compose -f compose_rocket_prj_addon.yml up -d --build  --force-recreate                  
```
> *Note:*{: style="color: orange;font-size:13px; "} <br>
> <small>our project is created in \app\[$env:PRJ_NAME_ARG] if environment is not set the default name is used</small>

3. Then in the container, in the newly created project directory(project_name_dir), execute first a command to **build** the project and then a command to **run** the project
```
cargo build --release			# Builds the project
cargo run --release			# Runs the rocket server
```
4. Now you can browse via the host in a browser to: [http://localhost:8000/](http://localhost:8000/)


<br>

----

### 2. Be-aware of the following:
- ***Project files are created in the container (/app)*** not on the host, there is a bind mount on the container: **/host/workdir** in the host you find it in: **/workdir** You can use this to copy the projects here, so they are available on the host.<br><br>
- The new project name directory(see step 1) will contain a ***rocket.toml*** file that holds the web settings, one of these setting is the:
***address = "0.0.0.0"*** this must be set to 0.0.0.0 to make sure the host can access the web site. 
Another important settings is for example: port, if you change this make sure to update also the 'compose_rust_cont.yml' file. Other settings are available as comments in the file(not all) <br><br>
- The config file and the template main are copied from the host in the directory:
 ***RustService\Rocket_customized*** <br><br>
- When running more than one web application on the same server make sure to use another web port in the Rocket.toml file and in the docker files(compose)<br><br>
- use: ```$env:PRJ_NAME_ARG ``` in Power-shell host command box to test the new directory that will be created

