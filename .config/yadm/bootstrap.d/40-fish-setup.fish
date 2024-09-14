#!/bin/fish

function info
    set_color -o
    echo -n (status basename)": "
    set_color normal
    echo $argv
end

set -Ux fish_greeting

# setup editor
info "Setup editor..."
set -l helix_path (command -s hx || command -s helix )

if [ -n $helix_path ]
    info "Found helix at '$helix_path'"
    set -Ux EDITOR $helix_path
    set -Ux VISUAL $helix_path
end

info "Install fisher plugins..."

fisher update </dev/null
