---
layout: default_c
RefPages:
 - howto_create_a_dev_container
--- 


# Rust <span style="color: #409EFF; font-size: 0.6em; font-style: italic;"> -  Docker Container</span>

## ℹ️ Introduction

This is a Rust base image with optional project extensions for Rust frameworks (Rocket & Warp currently). It contains:

1. Debian container with Rust.
1. Instructions for creating a local development container. 
1. Instructions for adding additional frameworks.

## 📁 Repository Structure

You can use this container to develop your Rust-based web application, When installing one of the web frameworks a project with a default sample web application will be created.
Directory structure:

- ***RustService*** contains the Rust base service and the extension docker files
- ***Howtos*** directory with explanations of installation procedures and other instructions         
- ***workdir*** directory that will be 'bind mounted' in Linux for projects (Note, Linux directories in small case)

# 🛠️ Setup Information

More documents can be found in the 'Howto" directory.

1. For creating and starting this Rust Docker container: open [**how to create this container**](./Howtos/howto_create_a_dev_container) 
1. To add framework projects, these instructions are also available in the same document, [**click here**](./Howtos/howto_create_a_dev_container#add-on_webrocket) to go directly to that paragraph.

<br>
<p align="center">
  <a href="https://nicojane.github.io/Docker-Template-Stacks-Home">
    <img src="assets/images/DTSfooter.svg" alt="DTS Template Stacks" width="400" />
  </a>
</p>

<span style="color: #6d757dff; font-size: 10px; font-style: italic;"> <br>
This file is part of:**Rust Template Stack**
Copyright (c) 2024 Nico Jan Eelhart. This source code is licensed under the MIT License found in the  'LICENSE.md' file in the root directory of this source tree.</span>
</small>
<div align="center"> ─── ✦ ─── </div>



<!--
<br><br><br>
# Table of content
* Table of Contents
{:toc}
-->