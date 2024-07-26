---
layout: default_c
RefPages:
 - howto_create_a_dev_container
--- 

<small>
_This file is part of: **Rust Template Stack**_
_Copyright (c) 2024 Nico Jan Eelhart_
_This source code is licensed under the MIT License found in the  'LICENSE.md' file in the root directory of this source tree._
</small>
<br><br>
<br>

# What Rust service

This is a Rust base image with optional project extensions for Rust frameworks (Rocket & Warp currently). It contains:

1. Debian container with Rust.
1. Instructions for creating a local development container. 
1. Instructions for adding additional frameworks.

You can use this container to develop your Rust-based web application, When installing one of the web frameworks a project with a default sample web application will be created.
Directory structure:

- ***RustService*** contains the Rust base service and the extension docker files
- ***Howtos*** directory with explanations of installation procedures and other instructions         
- ***workdir*** directory that will be 'bind mounted' in Linux for projects (Note, Linux directories in small case)

----

# Where to find more information
More documents can be found in the 'Howto" directory. 
1. For creating and starting this Rust Docker container: open [how to create a development container](./Howtos/howto_create_a_dev_container) 
1. To add framework projects, these instructions are also available in the same document, [click here](./Howtos/howto_create_a_dev_container#add-on_webrocket) to go directly to that paragraph.


----

### My personal folder concept structure
The containers are **maintained only here**. After creation or updating of the container, the container is copied to the program language folder inside the   **\_docker-template** folder. From there it can be used in projects, example:

<small style="display: block; margin-bottom: -18px;"><b><i>Folder concept</i></b></small>

|**language**| **Copy to Template Location** | **Use at Project Location**|
|:--------| :-------------- | :-------------------- |
|Rust    | \Rust\\_docker-template\ 		| \Rust\Projects\projectX |
|Python  | \Python\\_docker-template\ 	| \Python\Projects\ProjectY |

<small><i>Check the version of the template in the 'index.md' file (root)</i></small> <br><br>


> *Remark:*{: style="color: Grey;font-size:13px; "} <br>
> <small>Of course, you may change the container docker templates files for specific reasons but do this in the specific project folder! When a change is generic it should be added (merged) into the development container here.</small>


<br>


<!--
<br><br><br>
# Table of content
* Table of Contents
{:toc}
-->