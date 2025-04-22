# My Career Manager

This program helps me manage my career. Currently, it only handles my website: [www.eguefif.org](https://www.eguefif.org).
It can:
* Publish the website
* Update the profile
* add and edit the portfolio's projects
* Add and edit blog articles.

[image](./manager.gif)

# How it works
It is inspired by [local first](https://localfirstweb.dev/) kind of design. I want to simplify what I push online and keep all the logic locally.
The program is composed by:
* a local backend server written in Rust
* a Single Page Application for the interface

I don't have many dependencies. I designed the [HTTP server library](https://github.com/eguefif/webserv-rs/) and the [MD converter](https://github.com/eguefif/md_to_html/).
I use serde, chrono, and sqlite3 in the application. The server crate uses chrono and flate (for compression).

On the admin page, I can publish my website on render. It simply builds the website using my templates, copies everything into a repo, and pushes it to GitHub. [Render](https://render.com/) pull my repo and deploy the website.

If I want to preview, I push the __start preview__ button on my admin page. It starts a Docker with nginx, builds the website, and copies the files. I can click on the __see preview__ button and check what it looks like.

# What I designed
For learning purposes, I made my own tools. They are basic and not production-grade at all. Here is a list:
* [webserver library](https://github.com/eguefif/webserv-rs) that can handle chunked, compressed packets.
* [md to html converter](https://github.com/eguefif/md_to_html)
* template language and a renderer based on context
* a basic ORM that can insert, update, or retrieve data.
* a basic routing system (I've learn so much about how browser build their urls!)

