# Kode-WEBSERVER -- v1.1 -- 2024-01-09

Webserver completely written in Rust, and it also my first project ever made with Rust ;)
(thus why the code is really messy, dont judge me 🥲)
It can:

## Fetch html files to stream of a directory
The server fetches files from the "views" folder present in the same dir as the server's executable, it will fetch all the html files and use them to display to the website.
It can also fetch a 404.html file to display for the ERROR 404.

## TOADD/TODO(possibly in the future):

### Handle POST and GET requests differently;

### Handle data references on html;
for ex: {{ hello }} on the html file would turn onto a variable content with the name {{ hello }}.

### Handle SQL databases and also sessions.

All of this would be handled with a "custom scripting language" (which seems something large but possible, if I can lol).

_(19/03/24) Well after a couple of time being separated from this project, I've came to it again and saw the files, the "backuh" files are backup files from the ones in the directory I made incase I messed sum up, not rlly needed still due to main.rs functioning how it is supposed to_ 
