# Kode-WEBSERVER -- v1.1 -- 2024-01-09

Webserver completely written in Rust, and it also my first project ever made with Rust ;)

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
