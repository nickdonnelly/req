# req: command line HTTP client

`req` is a quick and easy cross platform HTTP client written in Rust. This project was more of an experiment than anything, but feel free to use it or contribute if you wish.

## Usage

Simply run: 

`req <options> [method] [url]`

For example:

`req get www.google.com`

If you wish to suppress the printing of the http headers:

`req --noheader www.google.com`

You can also only print the headers:

`req --headeronly www.google.com`
