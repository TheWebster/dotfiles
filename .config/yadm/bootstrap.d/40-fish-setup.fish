#!/bin/fish

set -l distro (cat /etc/os-release | sed -En 's/^ID=(.+)$/\1/p')

set -U fish_greeting

if [ "$distro" = artix ]
    fish_add_path -U /usr/lib/rustup/bin/
end

# setup editor
set -l helix_path (command -s hx || command -s helix )

if [ -n $helix_path ]
    set -Ux EDITOR $helix_path
    set -Ux VISUAL $helix_path
end

if ! functions -q fisher
    curl -sL https://raw.githubusercontent.com/jorgebucaran/fisher/main/functions/fisher.fish | source && fisher update
end
