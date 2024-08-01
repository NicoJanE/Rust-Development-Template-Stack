---
layout: default_c
RefPages:
 - howto_create_a_dev_container
--- 

<small>
<br><br>
_This file is part of: **Rust Template Stack**_
_Copyright (c) 2024 Nico Jan Eelhart_
_This source code is licensed under the MIT License found in the  'LICENSE.md' file in the root directory of this source tree._
</small>
<br><br>
<br>

# 1 Create and start the Rust container
This is the Rust Docker container setup. Containing instructions for creating, starting and customize the container.

<details closed>  
  <summary class="clickable-summary">
  <span  class="summary-icon"></span> 
  **Side note**: Docker call syntax
  </summary> 	<!-- On same line is failure, Don't indent the following Markdown lines!  -->
  
>### Docker call syntax 
<small> (***Skip this if you known docker basics***) </small><br>
**Take note: Docker calling context**
Because we use Docker files (Dockerfile and compose) with descriptive names, for example, **Dockerfile_Nodejs_React_Cont** instead of plain **Dockerfile**, this has an impact on the way Docker commands are run and called. For example, with a plain **Dockerfile**, we would use this command to call the Docker file in the **Docker Compose** file:
<br>
```
context: .
dockerfile: Dockefile
```
In our case, we cannot use the default name but have to specify the name we gave, thus:<br>
```     
build: 	    
context: .
dockerfile: Dockerfile_Nodejs_React_Cont	    
```
 The same applies for using the build command. With the default Dockerfile, you can use this:
```
 docker build 
 # This will assume a file: Dockerfile is available
```
With the named file, we have to use
```
 docker build -f MyDockerFileNameHere
```
The same applies for running the Compose file (use **-f** option)
</details>

## 1.1 The basic Rust container setup
To creat and start the Rust basic container, in Docker Desktop, execute the command below from the **RustService** directory:  

<details closed>  
  <summary class="clickable-summary">
  <span  class="summary-icon"></span> 
  **Side note**: Create Project from Template
  </summary> 	<!-- On same line is failure, Don't indent the following Markdown lines!  -->
  
>### Create Project from Template
>>  <small> ***Skipp this if you known how to deal with copy\customize docker files*** </small> <br>
>
> To adapt the template directory for your project, follow these steps. This guide assumes you’re using the React stack; if you’re working with a different stack (e.g., PHP, Rust), simply replace “React” with the stack name your are using.
> 1. Copy the whole directory to your project name:
`copy "React Development Template Stack" MyReactStack` <br> <br>
> 1. within your **MyReactStack** open the ***[name]Service*** directory <br><br>
*Warning*{: style="color: red;font-size:13px; "} <small>When using multiple containers, it's a good idea to rename the directory (for example, by adding a number) before proceeding. Otherwise, the containers will be grouped together, which is generally helpful, but this can lead to caching issues in certain container stacks, such as React. These issues may manifest as the same directories appearing in the container from a previous instance after running the **compose_nodejs_react_cont.yml** command. Caching problems can be quite troublesome in some Docker stack configurations</small> <br><br>
> 3. Customize the Dockerfiles: Since most Docker Compose setups involve a parent-child relationship (i.e., chaining), a change in one Dockerfile may require updates to all related files. Follow these steps:<br><br>
3.1  In the first compose_\* file change the **services name** to an appropriate name: <br>
```services:
webserver-nodejs-react:  # Change this ```<br> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; <!-- sorry for this --> 	
<small> <sup>*</sup>Always use lowercase!</small> <br><br>
3.2 The above **service name** may appear more than once in the same file, update these service names as well! <br><br>
3.3 Changes the **service name**  from step 3.1 in the other **compose_\* files**  <br><br>
3.4 Check the compose_\* files when it contain a **image name** than update this to your own image name:<br>
`` build:`` <br>
``     context: .  ``<br>
``     dockerfile: Dockerfile_Nodejs_React_Cont`` <br>
``       image: eelhart/react-base:latest      `` <br>
``		# Update above. i.e: [yourname/react-prjx]`` <br><br>
3.5 This **image name** may appear in other compose_\* files and other Dockerfile_\* files, updates these image names as well.
>
> 4 Lastly, update the ports to ensure that each host port is unique across all running containers. In your Docker Compose file, you might see this configuration: <br>
``ports:`` <br>
``target: 3001        # Container port.`` <br> 
`` published: 3002    # Host port, Make SURE it is unique    `` <br>
<br><small> Alternatively, the syntax might look like this (achieving the same result): </small><br>
`` ports:`` <br>
`` - "3002:3001"      # host:container`` <br><br>
> **Make sure that Host port: 3002 is not used by any other docker container or other services on your host!**
<br> <br>
</details>


```
# Create and start the Rust container
docker-compose -f compose_rust_cont.yml up -d --build --force-recreate
```

### <a name="add-on_webrocket"></a> 1.1 Optional add a Web Rocket project (using a add-on Docker compose file)
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
> <small>our project is created in \app\[$env:PRJ_NAME_ARG] if environment is not set a default name is used</small>

3. Then in the container, in the newly created project directory(project_name_dir), execute first a command to **build** the project and then a command to **run** the project
```
cargo build --release			# Builds the project
cargo run --release			# Runs the rocket server
```
### Setup Result
After this command you can open a browser in the host and surf to : [http://localhost:8000/](http://localhost:8000/)



### 1.2 Notes and things to be-aware of
- ***Project files are created in the container (/app)*** not on the host, there is a bind mount on the container: **/host/workdir** in the host you find it in: **/workdir** You can use this to copy the projects here, so they are available on the host.<br><br>
- The new project name directory(see step 1) will contain a ***rocket.toml*** file that holds the web settings, one of these setting is the:
***address = "0.0.0.0"*** this must be set to 0.0.0.0 to make sure the host can access the web site. 
Another important settings is for example: port, if you change this make sure to update also the 'compose_rust_cont.yml' file. Other settings are available as comments in the file(not all) <br><br>
- The config file and the template main are copied from the host in the directory:
 ***RustService\Rocket_customized*** <br><br>
- When running more than one web application on the same server make sure to use another web port in the Rocket.toml file and in the docker files(compose)<br><br>
- use: ```$env:PRJ_NAME_ARG ``` in Power-shell host command box to test the new directory that will be created

