# req

`req` is an quick, easy, environment-aware HTTP client written in Rust for your ANSI terminal.

  - Supports global environment variables for URLs, timeouts, request payloads, and more.
    - [`dotenv`](https://github.com/purpliminal/rust-dotenv) support: use `.env` files to set per-directory defaults so you don't have to type commands over and over again.
      - Lightweight and fast.

      # Environment Awareness

      There are two ways to use `req`'s environment awareness:
       - Set environment variables manually
        - Use `.env` files (this is better!)

        For examples about how to use `.env` files see the examples section.
        ### Valid environment variables
        | Environment Variable | Function                                                             | Valid attributes                                                      |
        |----------------------|----------------------------------------------------------------------|-----------------------------------------------------------------------|
        | `REQ_HTTP_METHOD`    | Sets the default HTTP method for requests.                           | `get`, `head`, `post`, `put`, `delete`, `options`, `connect`, `trace` |
        | `REQ_URI`            | Sets the default base URI for requests.                              | Any valid HTTP URI                                                    |
        | `REQ_TIMEOUT`        | Sets the timeout for requests (in milliseconds).                     | Any non-zero integer                                                  |
        | `REQ_PAYLOAD_FILE`   | Sets the default payload file for attaching to requests as the body. | Any filename                                                          |

# Examples
## Simple Requests
```sh
req get example.com              # GET http
req get http://example.com       # These are equivalent

req options https://example.com # OPTIONS https

req https://example.com          # GET requests are the default if you omit a verb
```

## Requests with bodies
`req` will try to automatically derive the `Content-Type` by looking at the extension of the file you provide. It
```sh
# Automatically derives 'Content-Type: application/json'
req post --body body_payload.json https://example.com   

# If it fails, it will use `Content-Type: application/octet-stream'
req post --body extensionless_file https://example.com

# You can also set the content type manually
req post --body extensionless_file -h "Content-Type" "image/png" https://example.com 

# Or any header!
req get --header HeaderName HeaderValue example.com

# You can also modify the timeout (default is 30 seconds):
req get --timeout 5000 google.com # GET request with 5 second timeout
```

## Example with `dotenv`/`.env`

Consider this `.env` file for a given directory:

```sh
REQ_URI=https://myproject.xyz
REQ_HTTP_METHOD=post
REQ_PAYLOAD_FILE=myrequest.json
```

The following are now all valid commands when run within the directory that contains the `.env` file:

```sh
req     # Run a POST request with myrequest.json as the body to https://myproject.xyz

req -h "Content-Type" "application/octet-stream" # Same as above, but overriding detected application/json

req put # Run a PUT request with myrequest.json as the body to https://myproject.xyz

req get google.com  # Run a GET to google

req put --body none # Run a POST request to https://myproject.xyz without a body
```
