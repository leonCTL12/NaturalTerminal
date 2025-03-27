#!/bin/sh

# Configure shortcuts
echo -n "Do you want to set up an alias 'n' for 'NaturalTerminal natural'? (y/n): "
read answer
if [ "$answer" = "y" ] || [ "$answer" = "Y" ]; then
  if [ -n "$BASH_VERSION" ]; then
    echo "bash found, version: $BASH_VERSION"
    config_file="$HOME/.bashrc"
    if [ -f "$config_file" ] && grep -q "^alias n=" "$config_file"; then
      echo "Warning: 'n' is already an alias in $config_file. You can add it manually with a different name."
    else
      echo "\n \n alias n='NaturalTerminal natural'" >> "$config_file"
      echo "Alias 'n' added to $config_file. Run 'source $config_file' or restart your terminal to use it."
    fi
  elif [ -n "$ZSH_VERSION" ]; then
    echo "zsh found, version: $ZSH_VERSION"
    config_file="$HOME/.zshrc"
    if [ -f "$config_file" ] && grep -q "^alias n=" "$config_file"; then
      echo "Warning: 'n' is already an alias in $config_file. You can add it manually with a different name."
    else
      echo "\n \n alias n='NaturalTerminal natural'" >> "$config_file"
      echo "Alias 'n' added to $config_file. Run 'source $config_file' or restart your terminal to use it."
    fi
  else
    echo "This script only supports bash and zsh. You can add the alias manually."
  fi
else
  echo "No alias added. Run the program with 'NaturalTerminal natural'."
fi

echo "Installation complete!"