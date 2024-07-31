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


<details closed>  
  <summary class="clickable-summary">
  <span  class="summary-icon"></span> 
  **Side note**: Personal maintenance instructions
  </summary> 	<!-- On same line is failure, Don't indent the following Markdown lines!  -->
  
>### Personal maintenance instructions
>The template containers are **maintained** only in the **DTS**. I copy these to a project directory and customize them there for the project. If the customization is generic, I will merge it into the DTS template project
>
> <small style="display: block; margin-bottom: -18px;"><b><i>Personal project structure</i></b></small> 
>
>  <small> **Docker-Template-Stacks (DTS)**</small> | <small> **Project Location** </small> | 
> :-------------- | :-------------------- |
> <small>DTS\PHP Development Template Stack\\ </small> | <small> \Php\Projects\projectX  </small>
> <small>DTS\Rust Template Stack\\ </small> 	| <small> \Rust\Projects\ProjectY </small>
>
><br>
> *Update:*{: style="color: Grey;font-size:13px; "} <small> these template central!</small>
</details>

<br>


<!--
<br><br><br>
# Table of content
* Table of Contents
{:toc}
-->