---
layout: default_c
RefPages:
 - howto_create_a_dev_container
--- 


# Rust Development <span style="color: #409EFF; font-size: 0.6em; font-style: italic;"> -  Docker Container</span>

![MIT License](https://img.shields.io/badge/License-MIT-green) ![Commercial Services Available](https://img.shields.io/badge/Services-Optional-blue)

## ℹ️ Introduction

This repository provides a Rust development environment using Docker containers, designed for building modern web applications. It includes:

1. **Debian-based Rust container** - Pre-configured development environment with the latest Rust toolchain.
2. **Web framework support** - Optional extensions for Rocket and Warp frameworks with sample projects.
3. **Local development setup** - Step-by-step instructions for creating and customizing your development container.
4. **Framework integration** - Easy-to-follow guides for adding additional Rust frameworks to your project.

Use this container to streamline your Rust web application development with minimal setup time.

## 📁 Repository Structure

You can use this container to develop your Rust-based web application, When installing one of the web frameworks a project with a default sample web application will be created.
Directory structure:

- ***RustService*** contains the Rust base service and the extension docker files
- ***Howtos*** directory with explanations of installation procedures and other instructions         
- ***workdir*** directory that will be 'bind mounted' in Linux for projects (Note, Linux directories in small case)

# 🛠️ Setup Information

More documents can be found in the 'Howto" directory.

1. For creating and starting this Rust Docker container open [**basic setup**](./Howtos/howto_create_a_dev_container)
1. To add framework projects, these instructions are also available in the same document, [**click here**](./Howtos/howto_create_a_dev_container#add-on_webrocket) to go directly to that paragraph.

<br>
<small> <i><b>License</b><br>This file is part of: <b>Rust Template Stack</b>  Copyright (c) 2025-2026 Nico Jan Eelhart.This repository is <a href="MIT-license.md">MIT licensed</a>and free to use. For optional commercial support, customization, training, or long-term maintenance, see <a href="COMMERCIAL.md">COMMERCIAL.md</a></i>
</small>

<br><br>
<p align="center">
  <a href="https://nicojane.github.io/Docker-Template-Stacks-Home">
    <img src="assets/images/DTSfooter.svg" alt="DTS Template Stacks" width="400" />
  </a>
</p>

<br>
<center>─── ✦ ───</center>



<!--
<br><br><br>
# Table of content
* Table of Contents
{:toc}
-->