#!/bin/fish

# setup editor
set -l helix_path (command -s hx || command -s helix )

if [ -n $helix_path ]
    info "Found helix at '$helix_path'"
    set -Ux EDITOR $helix_path
    set -Ux VISUAL $helix_path
end

curl -sL https://raw.githubusercontent.com/jorgebucaran/fisher/main/functions/fisher.fish | source && fisher update
