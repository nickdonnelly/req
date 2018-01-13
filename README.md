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
| `REQ_MAX_REDIRECTS`  | Sets the maximum number of redirects.                                | Any integer at least zero (-1 for infinite redirects).                |
| `REQ_ENCODING`       | Sets the default encoding for the request body.                      | `none`, `base64`                                                      |

# Examples
## Simple Requests
```sh
req get example.com              # GET http
req get http://example.com       # These are equivalent

req options https://example.com # OPTIONS https

req https://example.com          # GET requests are the default if you omit a verb

req get http://example.com/redirect --max-redirects 3 # Follow custom number of redirects

req get http://example.com/redirect --max-redirects -1 # Follow infinite number of redirects
```

## Custom Output
You can determine what `req` prints to stdout and in what order.
```sh
# Prints only the headers sent in the request and the headers of the response.
req post --body some_body.json --print request-headers --print headers https://example.com

# Only show the response time.
req get google.com --print response-time 
```

Valid print options are: `body`, `headers`, `status`, `config`, `request-headers`, and `response-time`.

## Requests with bodies and headers
`req` will try to automatically derive the `Content-Type` by looking at the extension of the file you provide. It
```sh
# Automatically derives 'Content-Type: application/json'
req post --body body_payload.json https://example.com   

# If it fails, it will use `Content-Type: application/octet-stream'
req post --body extensionless_file https://example.com

# You can also set the content type manually
req post --body extensionless_file -h "Content-Type" "image/png" https://example.com 

# Encode your body automatically
req post --body file.png --encoding base64 https://example.com

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

## Other Commands

One of the most versatile `req` commands is `req show`. This command allows you to view information about your configuration or potential requests without actually needing to fire them. 

For example, if you wanted to see what a payload would look like **without having to actually fire a request with that payload attached**, you can run `req show payload [PAYLOAD_FILE]` to print exactly what would be applied to your request.

You can also use `show env` to see how req sees your environment.


# Compiling and Running Tests

## Compiling
To compile the project, simply run `cargo build --release`. 

## Tests
To run the test suite, you have to pass the `--test-threads=1` flag to the test executable. Cargo will run tests in parallel by default, but we can't do that in this case so we have to use only one thread. This is because some of the tests play with environment variables, so they can't run in parallel or they would step on one another's toes. 
