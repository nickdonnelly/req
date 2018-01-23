
@('req', './req') | %{
    Register-ArgumentCompleter -Native -CommandName $_ -ScriptBlock {
        param($wordToComplete, $commandAst, $cursorPosition)

        $command = '_req'
        $commandAst.CommandElements |
            Select-Object -Skip 1 |
            %{
                switch ($_.ToString()) {

                    'Req' {
                        $command += '_Req'
                        break
                    }

                    'get' {
                        $command += '_get'
                        break
                    }

                    'post' {
                        $command += '_post'
                        break
                    }

                    'put' {
                        $command += '_put'
                        break
                    }

                    'options' {
                        $command += '_options'
                        break
                    }

                    'head' {
                        $command += '_head'
                        break
                    }

                    'trace' {
                        $command += '_trace'
                        break
                    }

                    'connect' {
                        $command += '_connect'
                        break
                    }

                    'delete' {
                        $command += '_delete'
                        break
                    }

                    'patch' {
                        $command += '_patch'
                        break
                    }

                    'show' {
                        $command += '_show'
                        break
                    }

                    'payload' {
                        $command += '_payload'
                        break
                    }

                    'env' {
                        $command += '_env'
                        break
                    }

                    'help' {
                        $command += '_help'
                        break
                    }

                    'socket' {
                        $command += '_socket'
                        break
                    }

                    default { 
                        break
                    }
                }
            }

        $completions = @()

        switch ($command) {

            '_req' {
                $completions = @('get', 'post', 'put', 'options', 'head', 'trace', 'connect', 'delete', 'patch', 'show', 'socket', 'help', '-V', '-b', '-e', '-r', '-h', '-f', '-t', '-p', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--timeout', '--print')
            }

            '_req_get' {
                $completions = @('-V', '-b', '-e', '-r', '-h', '-f', '-p', '-t', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--print', '--timeout')
            }

            '_req_post' {
                $completions = @('-V', '-b', '-e', '-r', '-h', '-f', '-p', '-t', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--print', '--timeout')
            }

            '_req_put' {
                $completions = @('-V', '-b', '-e', '-r', '-h', '-f', '-p', '-t', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--print', '--timeout')
            }

            '_req_options' {
                $completions = @('-V', '-b', '-e', '-r', '-h', '-f', '-p', '-t', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--print', '--timeout')
            }

            '_req_head' {
                $completions = @('-V', '-b', '-e', '-r', '-h', '-f', '-p', '-t', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--print', '--timeout')
            }

            '_req_trace' {
                $completions = @('-V', '-b', '-e', '-r', '-h', '-f', '-p', '-t', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--print', '--timeout')
            }

            '_req_connect' {
                $completions = @('-V', '-b', '-e', '-r', '-h', '-f', '-p', '-t', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--print', '--timeout')
            }

            '_req_delete' {
                $completions = @('-V', '-b', '-e', '-r', '-h', '-f', '-p', '-t', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--print', '--timeout')
            }

            '_req_patch' {
                $completions = @('-V', '-b', '-e', '-r', '-h', '-f', '-p', '-t', '--help', '--version', '--body', '--encoding', '--body-prefix', '--max-redirects', '--header', '--header-file', '--print', '--timeout')
            }

            '_req_show' {
                $completions = @('payload', 'env', 'help', '-h', '-V', '--help', '--version')
            }

            '_req_show_payload' {
                $completions = @('-h', '-V', '-e', '--help', '--version', '--encoding', '--body-prefix')
            }

            '_req_show_env' {
                $completions = @('-h', '-V', '--help', '--version')
            }

            '_req_show_help' {
                $completions = @('-h', '-V', '--help', '--version')
            }

            '_req_socket' {
                $completions = @('-h', '-V', '-c', '-m', '-r', '--help', '--version', '--response-code', '--response-mode', '--response')
            }

            '_req_help' {
                $completions = @('-h', '-V', '--help', '--version')
            }

        }

        $completions |
            ?{ $_ -like "$wordToComplete*" } |
            Sort-Object |
            %{ New-Object System.Management.Automation.CompletionResult $_, $_, 'ParameterValue', $_ }
    }
}
