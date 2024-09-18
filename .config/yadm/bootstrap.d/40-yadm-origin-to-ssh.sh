#!/bin/sh

echo "Updating yadm repo URL to use SSH..."

yadm remote set-url origin "ssh://git@githubTheWebster/TheWebster/dotfiles"

if [ ! -f ~/.ssh/githubTheWebster ]
then
  ssh-keygen -f ~/.ssh/githubTheWebster
fi
