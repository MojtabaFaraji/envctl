# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_envctl_global_optspecs
    string join \n c/config= profile= v/verbose q/quiet y/yes dry-run no-color o/output= no-input system debug h/help V/version
end

function __fish_envctl_needs_command
    # Figure out if the current invocation already has a command.
    set -l cmd (commandline -opc)
    set -e cmd[1]
    argparse -s (__fish_envctl_global_optspecs) -- $cmd 2>/dev/null
    or return
    if set -q argv[1]
        # Also print the command, so this can be used to figure out what it is.
        echo $argv[1]
        return 1
    end
    return 0
end

function __fish_envctl_using_subcommand
    set -l cmd (__fish_envctl_needs_command)
    test -z "$cmd"
    and return 1
    contains -- $cmd[1] $argv
end

complete -c envctl -n "__fish_envctl_needs_command" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_needs_command" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_needs_command" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_needs_command" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_needs_command" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_needs_command" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_needs_command" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_needs_command" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_needs_command" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_needs_command" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_needs_command" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_needs_command" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_needs_command" -s V -l version -d 'Print version'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "init" -d 'Initialize config dir and defaults'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "add" -d 'Create a new environment'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "remove" -d 'Delete one or more environments'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "edit" -d 'Edit an environment (open $EDITOR or mutate via flags)'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "view" -d 'Show a single environment in full'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "list" -d 'List environments'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "set" -d 'Set a variable (KEY=VALUE or KEY VALUE)'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "unset" -d 'Unset one or more variables'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "rename" -d 'Rename an environment'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "clone" -d 'Clone (duplicate) an environment'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "import" -d 'Import an environment from file or stdin'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "export" -d 'Export an environment to file or stdout'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "diff" -d 'Diff two environments'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "validate" -d 'Validate environment(s)'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "activate" -d 'Emit shell code to activate an environment'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "deactivate" -d 'Emit shell code to deactivate the active environment'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "which" -d 'Show the active environment'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "completion" -d 'Generate shell completions'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "secret" -d 'Manage secrets in the system keyring'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "version" -d 'Print version'
complete -c envctl -n "__fish_envctl_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c envctl -n "__fish_envctl_using_subcommand init" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand init" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand init" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand init" -l force
complete -c envctl -n "__fish_envctl_using_subcommand init" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand init" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand init" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand init" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand init" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand init" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand init" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand init" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand init" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand add" -l description -r
complete -c envctl -n "__fish_envctl_using_subcommand add" -l var -r
complete -c envctl -n "__fish_envctl_using_subcommand add" -l path-prepend -r
complete -c envctl -n "__fish_envctl_using_subcommand add" -l path-append -r
complete -c envctl -n "__fish_envctl_using_subcommand add" -l alias -r
complete -c envctl -n "__fish_envctl_using_subcommand add" -l function -r
complete -c envctl -n "__fish_envctl_using_subcommand add" -l cwd -r
complete -c envctl -n "__fish_envctl_using_subcommand add" -l tags -r
complete -c envctl -n "__fish_envctl_using_subcommand add" -l from -r -F
complete -c envctl -n "__fish_envctl_using_subcommand add" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand add" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand add" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand add" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand add" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand add" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand add" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand add" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand add" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand add" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand add" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand add" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand remove" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand remove" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand remove" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand remove" -l force
complete -c envctl -n "__fish_envctl_using_subcommand remove" -l dry-run
complete -c envctl -n "__fish_envctl_using_subcommand remove" -l no-backup
complete -c envctl -n "__fish_envctl_using_subcommand remove" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand remove" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand remove" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand remove" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand remove" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand remove" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand remove" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand remove" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l set -r
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l unset -r
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l description -r
complete -c envctl -n "__fish_envctl_using_subcommand edit" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand edit" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l editor
complete -c envctl -n "__fish_envctl_using_subcommand edit" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand edit" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand edit" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand edit" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand edit" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand view" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand view" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand view" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand view" -l raw
complete -c envctl -n "__fish_envctl_using_subcommand view" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand view" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand view" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand view" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand view" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand view" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand view" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand view" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand view" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand list" -s f -l filter -r
complete -c envctl -n "__fish_envctl_using_subcommand list" -l sort -r
complete -c envctl -n "__fish_envctl_using_subcommand list" -s l -l limit -r
complete -c envctl -n "__fish_envctl_using_subcommand list" -l offset -r
complete -c envctl -n "__fish_envctl_using_subcommand list" -l tags -r
complete -c envctl -n "__fish_envctl_using_subcommand list" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand list" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand list" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand list" -l all
complete -c envctl -n "__fish_envctl_using_subcommand list" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand list" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand list" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand list" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand list" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand list" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand list" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand list" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand list" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand set" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand set" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand set" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand set" -l secret
complete -c envctl -n "__fish_envctl_using_subcommand set" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand set" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand set" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand set" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand set" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand set" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand set" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand set" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand set" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand unset" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand unset" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand unset" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand unset" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand unset" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand unset" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand unset" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand unset" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand unset" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand unset" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand unset" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand unset" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand rename" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand rename" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand rename" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand rename" -l force
complete -c envctl -n "__fish_envctl_using_subcommand rename" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand rename" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand rename" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand rename" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand rename" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand rename" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand rename" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand rename" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand rename" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand clone" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand clone" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand clone" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand clone" -l force
complete -c envctl -n "__fish_envctl_using_subcommand clone" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand clone" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand clone" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand clone" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand clone" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand clone" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand clone" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand clone" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand clone" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand import" -l name -r
complete -c envctl -n "__fish_envctl_using_subcommand import" -l format -r -f -a "auto\t''
dotenv\t''
json\t''
yaml\t''
toml\t''"
complete -c envctl -n "__fish_envctl_using_subcommand import" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand import" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand import" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand import" -l overwrite
complete -c envctl -n "__fish_envctl_using_subcommand import" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand import" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand import" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand import" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand import" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand import" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand import" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand import" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand import" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand export" -l format -r -f -a "dotenv\t''
json\t''
yaml\t''
toml\t''"
complete -c envctl -n "__fish_envctl_using_subcommand export" -l file -r -F
complete -c envctl -n "__fish_envctl_using_subcommand export" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand export" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand export" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand export" -l reveal
complete -c envctl -n "__fish_envctl_using_subcommand export" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand export" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand export" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand export" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand export" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand export" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand export" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand export" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand export" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand diff" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand diff" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand diff" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand diff" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand diff" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand diff" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand diff" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand diff" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand diff" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand diff" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand diff" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand diff" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand validate" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand validate" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand validate" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand validate" -l all
complete -c envctl -n "__fish_envctl_using_subcommand validate" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand validate" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand validate" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand validate" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand validate" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand validate" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand validate" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand validate" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand validate" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l shell -r -f -a "bash\t''
zsh\t''
fish\t''"
complete -c envctl -n "__fish_envctl_using_subcommand activate" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand activate" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l no-functions
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l no-aliases
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l no-cwd
complete -c envctl -n "__fish_envctl_using_subcommand activate" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand activate" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand activate" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand activate" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand activate" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand deactivate" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand which" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand which" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand which" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand which" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand which" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand which" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand which" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand which" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand which" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand which" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand which" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand which" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand completion" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand completion" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand completion" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand completion" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand completion" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand completion" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand completion" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand completion" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand completion" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand completion" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand completion" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand completion" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -f -a "set" -d 'Store a secret'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -f -a "get" -d 'Retrieve a secret (masked by default)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -f -a "rm" -d 'Remove a secret'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -f -a "list" -d 'List known secrets (names only)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and not __fish_seen_subcommand_from set get rm list help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -l from-stdin
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from set" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -l reveal
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from get" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from rm" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from help" -f -a "set" -d 'Store a secret'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from help" -f -a "get" -d 'Retrieve a secret (masked by default)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from help" -f -a "rm" -d 'Remove a secret'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from help" -f -a "list" -d 'List known secrets (names only)'
complete -c envctl -n "__fish_envctl_using_subcommand secret; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c envctl -n "__fish_envctl_using_subcommand version" -s c -l config -d 'Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)' -r -F
complete -c envctl -n "__fish_envctl_using_subcommand version" -l profile -d 'Named config profile (alternate config dir suffix)' -r
complete -c envctl -n "__fish_envctl_using_subcommand version" -s o -l output -d 'Output format' -r -f -a "table\t''
json\t''
yaml\t''
plain\t''"
complete -c envctl -n "__fish_envctl_using_subcommand version" -s v -l verbose -d 'Verbose logging (repeat for more)'
complete -c envctl -n "__fish_envctl_using_subcommand version" -s q -l quiet -d 'Quiet mode (errors only)'
complete -c envctl -n "__fish_envctl_using_subcommand version" -s y -l yes -d 'Assume yes to prompts'
complete -c envctl -n "__fish_envctl_using_subcommand version" -l dry-run -d 'Print actions without applying'
complete -c envctl -n "__fish_envctl_using_subcommand version" -l no-color -d 'Disable color'
complete -c envctl -n "__fish_envctl_using_subcommand version" -l no-input -d 'Non-interactive: fail instead of prompting'
complete -c envctl -n "__fish_envctl_using_subcommand version" -l system -d 'Operate on the system-wide store (requires root)'
complete -c envctl -n "__fish_envctl_using_subcommand version" -l debug -d 'Debug logging'
complete -c envctl -n "__fish_envctl_using_subcommand version" -s h -l help -d 'Print help'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "init" -d 'Initialize config dir and defaults'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "add" -d 'Create a new environment'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "remove" -d 'Delete one or more environments'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "edit" -d 'Edit an environment (open $EDITOR or mutate via flags)'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "view" -d 'Show a single environment in full'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "list" -d 'List environments'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "set" -d 'Set a variable (KEY=VALUE or KEY VALUE)'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "unset" -d 'Unset one or more variables'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "rename" -d 'Rename an environment'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "clone" -d 'Clone (duplicate) an environment'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "import" -d 'Import an environment from file or stdin'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "export" -d 'Export an environment to file or stdout'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "diff" -d 'Diff two environments'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "validate" -d 'Validate environment(s)'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "activate" -d 'Emit shell code to activate an environment'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "deactivate" -d 'Emit shell code to deactivate the active environment'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "which" -d 'Show the active environment'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "completion" -d 'Generate shell completions'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "secret" -d 'Manage secrets in the system keyring'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "version" -d 'Print version'
complete -c envctl -n "__fish_envctl_using_subcommand help; and not __fish_seen_subcommand_from init add remove edit view list set unset rename clone import export diff validate activate deactivate which completion secret version help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c envctl -n "__fish_envctl_using_subcommand help; and __fish_seen_subcommand_from secret" -f -a "set" -d 'Store a secret'
complete -c envctl -n "__fish_envctl_using_subcommand help; and __fish_seen_subcommand_from secret" -f -a "get" -d 'Retrieve a secret (masked by default)'
complete -c envctl -n "__fish_envctl_using_subcommand help; and __fish_seen_subcommand_from secret" -f -a "rm" -d 'Remove a secret'
complete -c envctl -n "__fish_envctl_using_subcommand help; and __fish_seen_subcommand_from secret" -f -a "list" -d 'List known secrets (names only)'
