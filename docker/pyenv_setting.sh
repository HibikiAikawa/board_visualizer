#!/bin/sh

export PYENV_ROOT="$HOME/.pyenv"
export PATH="/root/.local/bin:$PYENV_ROOT/bin:$PATH"
eval "$(pyenv init --path)"
eval "$(pyenv virtualenv-init -)"
