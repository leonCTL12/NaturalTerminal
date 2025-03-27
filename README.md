# Natural Terminal

Tired of memorizing complex terminal commands? Natural Terminal lets you interact with your terminal using plain, natural language. Simply say what you want—like "list all files" or "delete old logs"—and it translates your words into the right commands using a powerful language model (LLM).

Choose your setup: run it locally with Ollama for privacy and speed, or tap into remote models via OpenRouter for maximum flexibility. Pick any model that suits your needs, from lightweight locals to cutting-edge cloud options. Say goodbye to syntax struggles and hello to a smarter terminal!

<aside>
⚠️

For now, this tool supports macOS only. Linux may be supported in the future.

</aside>

# Prerequisites

- **Local LLM**: To use a local language model, ensure Ollama is installed and running on your device. Visit https://ollama.com/ to download it.
- **OpenRouter**: To use remote models via OpenRouter, you’ll need an API key. Create one at https://openrouter.ai/.

# Quick Start

1. Clone the repository to your machine.
2. Run the installation script, Zsh (recommended, macOS default)
    
    ```bash
    zsh install.sh
    ```
    
    or Bash
    
    ```bash
    bash install.sh
    ```
    
3. Run this to set things up! Follow the instructions:
    
    ```bash
    naturalterminal init
    ```
    
4. Done. Start using Natural Terminal by simply typing
    
    ```bash
    n
    ```
    
    or the full command if you chose not to set up the alias
    
    ```bash
    naturalterminal natural
    ```
