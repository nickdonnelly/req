function __fish_using_command
    set cmd (commandline -opc)
    if [ (count $cmd) -eq (count $argv) ]
        for i in (seq (count $argv))
            if [ $cmd[$i] != $argv[$i] ]
                return 1
            end
        end
        return 0
    end
    return 1
end

complete -c req -n "__fish_using_command req" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req" -f -a "get"
complete -c req -n "__fish_using_command req" -f -a "post"
complete -c req -n "__fish_using_command req" -f -a "put"
complete -c req -n "__fish_using_command req" -f -a "options"
complete -c req -n "__fish_using_command req" -f -a "head"
complete -c req -n "__fish_using_command req" -f -a "trace"
complete -c req -n "__fish_using_command req" -f -a "connect"
complete -c req -n "__fish_using_command req" -f -a "delete"
complete -c req -n "__fish_using_command req" -f -a "patch"
complete -c req -n "__fish_using_command req" -f -a "show" -d 'Show the specified resource.'
complete -c req -n "__fish_using_command req" -f -a "socket" -d 'Launch a socket on the given port to read incoming requests easily.'
complete -c req -n "__fish_using_command req" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c req -n "__fish_using_command req get" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req get" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req get" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req get" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req get" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req get" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req get" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req get" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req get" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req get" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req post" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req post" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req post" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req post" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req post" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req post" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req post" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req post" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req post" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req post" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req put" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req put" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req put" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req put" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req put" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req put" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req put" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req put" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req put" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req put" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req options" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req options" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req options" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req options" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req options" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req options" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req options" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req options" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req options" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req options" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req head" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req head" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req head" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req head" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req head" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req head" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req head" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req head" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req head" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req head" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req trace" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req trace" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req trace" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req trace" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req trace" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req trace" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req trace" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req trace" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req trace" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req trace" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req connect" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req connect" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req connect" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req connect" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req connect" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req connect" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req connect" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req connect" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req connect" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req connect" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req delete" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req delete" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req delete" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req delete" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req delete" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req delete" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req delete" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req delete" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req delete" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req delete" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req patch" -s b -l body -d 'Specify a file to use as the body of the request.'
complete -c req -n "__fish_using_command req patch" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req patch" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req patch" -s r -l max-redirects -d 'The maximum number of redirects to follow. Set this to -1 for infinite follows.'
complete -c req -n "__fish_using_command req patch" -s h -l header -d 'Specify a custom header. Overrides headers in --header-file.Use the format "Header Name" "Value"'
complete -c req -n "__fish_using_command req patch" -s f -l header-file -d 'Specify multiple headers in a file (one per line, same format as --header).Use the value \'none\' to ignore the environment variable.'
complete -c req -n "__fish_using_command req patch" -s p -l print -d 'Explicitly decide which parts of the response to print.' -r -f -a "body headers request-headers status response-time config"
complete -c req -n "__fish_using_command req patch" -s t -l timeout -d 'Specify the request timeout in millseconds.'
complete -c req -n "__fish_using_command req patch" -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req patch" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req show" -s h -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req show" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req show" -f -a "payload" -d 'Displays how a payload would look when attached to a request. This is useful for things that will look like UTF-8 text.'
complete -c req -n "__fish_using_command req show" -f -a "env" -d 'Displays the current req values from the environment. Prioritizes .env over other values.'
complete -c req -n "__fish_using_command req show" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c req -n "__fish_using_command req show payload" -s e -l encoding -d 'Automatically encode the payload using this type.' -r -f -a "none base64"
complete -c req -n "__fish_using_command req show payload" -l body-prefix -d 'Append a prefix to the request body (added after encoding if encoding was applied).'
complete -c req -n "__fish_using_command req show payload" -s h -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req show payload" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req show env" -s h -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req show env" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req show help" -s h -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req show help" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req socket" -s c -l response-code -d 'The response code (as a number) you would like to send all requests.'
complete -c req -n "__fish_using_command req socket" -s m -l response-mode -d 'The mode for responses the socket gives' -r -f -a "talkback literal"
complete -c req -n "__fish_using_command req socket" -s r -l response -d 'A literal string you would like the socket to respond with.'
complete -c req -n "__fish_using_command req socket" -s h -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req socket" -s V -l version -d 'Prints version information'
complete -c req -n "__fish_using_command req help" -s h -l help -d 'Prints help information'
complete -c req -n "__fish_using_command req help" -s V -l version -d 'Prints version information'
