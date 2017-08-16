# req: command line HTTP client

`req` is a quick and easy cross platform HTTP client written in Rust. This project was more of an experiment than anything, but feel free to use it or contribute if you wish.

## Usage and options

For normal use, simply run: 

`req <options> [method] [url]`

For example, to run a GET request to `google.com`:

`req get www.google.com`

If you wish to suppress the printing of the http headers:

`req --noheader get www.google.com`

You can also only print the headers:

`req --headeronly post www.google.com`

If you want to modify the headers yourself, use:

`req --customheaders put www.google.com`

To follow redirects, use (note: currently following redirects only sends the payload and the headers you specify to the initial host, not the redirected host.):

`req --follow-redirects get www.google.com`

If you want to manually enter text into the request body, use:

`req --enter-payload get www.google.com`

If you want to use a payload from a file, use:
`req --payload-file filename.ext get www.google.com`



## Supported methods

All normal HTTP request methods are supported:

* GET
* HEAD (Note: Currently due to a bug, this will work but the program won't terminate. Just press ctrl+C when the headers have printed out. This will be fixed in the future)
* POST
* PUT
* DELETE
* CONNECT
* OPTIONS
* PATCH
* TRACE
