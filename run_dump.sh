#!/bin/bash

# Verifica se o Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "Rust não encontrado! Instalando..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo "Compilando o projeto..."
cargo build --release

# Verifica se a compilação foi bem-sucedida
if [ ! -f ./target/release/Dump_mem ]; then
    echo "Erro: O binário não foi gerado!"
    exit 1
fi

echo "Executando o dump de memória..."
sudo ./target/release/Dump_mem
