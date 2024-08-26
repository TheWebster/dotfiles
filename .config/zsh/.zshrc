#!/bin/zsh
# .zshrc

[[ -f $HOME/.cargo/env ]] && source $HOME/.cargo/env
[[ -f $HOME/.nvm/nvm.sh ]] && source $HOME/.nvm/nvm.sh

[[ -f $ZDOTDIR/paths ]] && echo paths

[[ -f $ZDOTDIR/omz.zsh ]] && source $ZDOTDIR/omz.zsh

[[ -f $ZDOTDIR/alias.sh ]] && source $ZDOTDIR/alias.sh

[[ -f $ZDOTDIR/env.sh ]] && source $ZDOTDIR/env.sh
