# Natural Terminal

Tired of memorizing complex terminal commands? Natural Terminal lets you interact with your terminal using plain, natural language. Simply say what you want—like "list all files" or "delete old logs"—and it translates your words into the right commands using a powerful language model (LLM).

Choose your setup: run it locally with Ollama for privacy and speed, or tap into remote models via OpenRouter for maximum flexibility. Pick any model that suits your needs, from lightweight locals to cutting-edge cloud options. Say goodbye to syntax struggles and hello to a smarter terminal!

<aside>
⚠️

For now, this tool supports macOS only. Linux may be supported in the future.

</aside>

# Prerequisites

- **Homebrew**: This tool is distributed via Homebrew. Ensure it’s installed by visiting https://brew.sh/.
- **Local LLM**: To use a local language model, ensure Ollama is installed and running on your device. Visit https://ollama.com/ to download it.
- **OpenRouter**: To use remote models via OpenRouter, you’ll need an API key. Create one at https://openrouter.ai/.

# Quick Start

1. Install via Homebrew:
   ```bash
   brew tap leonCTL12/homebrew-naturalterminal
   brew install leonCTL12/homebrew-naturalterminal/naturalterminal
   ```
2. (Optional) Set up the alias 'n':
   
    For zsh (default for mac, recommended)
    ```bash
    zsh /opt/homebrew/Cellar/naturalterminal/1.0.0/setup-alias.sh
    ```
   For bash
   ```bash
    /opt/homebrew/Cellar/naturalterminal/1.0.0/setup-alias.sh
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
5. To update (in the future)
   ```bash
   brew update
   brew upgrade leonCTL12/homebrew-naturalterminal/naturalterminal
   ```
