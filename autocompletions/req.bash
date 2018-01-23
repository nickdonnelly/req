_req() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            req)
                cmd="req"
                ;;
            
            connect)
                cmd+="__connect"
                ;;
            delete)
                cmd+="__delete"
                ;;
            env)
                cmd+="__env"
                ;;
            get)
                cmd+="__get"
                ;;
            head)
                cmd+="__head"
                ;;
            help)
                cmd+="__help"
                ;;
            options)
                cmd+="__options"
                ;;
            patch)
                cmd+="__patch"
                ;;
            payload)
                cmd+="__payload"
                ;;
            post)
                cmd+="__post"
                ;;
            put)
                cmd+="__put"
                ;;
            show)
                cmd+="__show"
                ;;
            socket)
                cmd+="__socket"
                ;;
            trace)
                cmd+="__trace"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        req)
            opts=" -V -b -e -r -h -f -t -p  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --timeout --print  <URI>  get post put options head trace connect delete patch show socket help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        
        req__connect)
            opts=" -V -b -e -r -h -f -p -t  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --print --timeout  <URI> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__delete)
            opts=" -V -b -e -r -h -f -p -t  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --print --timeout  <URI> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__get)
            opts=" -V -b -e -r -h -f -p -t  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --print --timeout  <URI> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__head)
            opts=" -V -b -e -r -h -f -p -t  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --print --timeout  <URI> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__options)
            opts=" -V -b -e -r -h -f -p -t  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --print --timeout  <URI> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__patch)
            opts=" -V -b -e -r -h -f -p -t  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --print --timeout  <URI> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__post)
            opts=" -V -b -e -r -h -f -p -t  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --print --timeout  <URI> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__put)
            opts=" -V -b -e -r -h -f -p -t  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --print --timeout  <URI> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__show)
            opts=" -h -V  --help --version   payload env help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__show__env)
            opts=" -h -V  --help --version  <VARIABLE> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__show__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__show__payload)
            opts=" -h -V -e  --help --version --encoding --body-prefix  <PAYLOAD_FILE> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__socket)
            opts=" -h -V -c -m -r  --help --version --response-code --response-mode --response  <PORT> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --response-code)
                    COMPREPLY=("<RESPONSE_CODE>")
                    return 0
                    ;;
                    -c)
                    COMPREPLY=("<RESPONSE_CODE>")
                    return 0
                    ;;
                --response-mode)
                    COMPREPLY=($(compgen -W "talkback literal" -- ${cur}))
                    return 0
                    ;;
                    -m)
                    COMPREPLY=($(compgen -W "talkback literal" -- ${cur}))
                    return 0
                    ;;
                --response)
                    COMPREPLY=("<RESPONSE>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<RESPONSE>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        req__trace)
            opts=" -V -b -e -r -h -f -p -t  --help --version --body --encoding --body-prefix --max-redirects --header --header-file --print --timeout  <URI> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --body)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                    -b)
                    COMPREPLY=("<PAYLOAD_FILE>")
                    return 0
                    ;;
                --encoding)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "none base64" -- ${cur}))
                    return 0
                    ;;
                --body-prefix)
                    COMPREPLY=("<PREFIX>")
                    return 0
                    ;;
                --max-redirects)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                    -r)
                    COMPREPLY=("<MAX_REDIRECTS>")
                    return 0
                    ;;
                --header)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                    -h)
                    COMPREPLY=("<HEADER>...")
                    return 0
                    ;;
                --header-file)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                    -f)
                    COMPREPLY=("<FILE>")
                    return 0
                    ;;
                --print)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -W "body headers request-headers status response-time config" -- ${cur}))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                    -t)
                    COMPREPLY=("<TIMEOUT>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
    esac
}

complete -F _req -o bashdefault -o default req
