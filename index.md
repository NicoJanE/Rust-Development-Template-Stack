---
layout: default
--- 

_This file is part of: PHP Development Template Stack_
_Copyright (c) 2024 Nico Jan Eelhart_

_This source code is licensed under the MIT License found in the  'LICENSE.md' file in the root directory of this source tree._
<br><br>

# What Rust service

This is a Rust base image with optional extensions for the Rust frameworks (Rocket & Warp currently). It contains:

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
1. For creating this Rust Docker container: open [how to create a development container](./Howtos/Howto_Create_A_Dev_container) 
1. To add framework projects, these instructions are also available in the same document, see the document on [how to create a development container](./Howtos/Howto_Create_A_Dev_container) 

<br>

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
<small><i>Version: 1.03 release 1 version June 19 2024 </i></small>

<!--
<br><br><br>
# Table of content
* Table of Contents
{:toc}
-->