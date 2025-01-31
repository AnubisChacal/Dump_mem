# Dump\_mem

## 📝 Descrição

Dump\_mem é um utilitário em Rust para extrair um dump completo da memória RAM de um sistema Linux. O programa divide automaticamente o dump em partes menores se a RAM for maior que 16GB e exibe uma barra de progresso durante o processo.

## 🚀 Funcionalidades

- Extração da memória RAM diretamente de `/dev/mem`
- Divisão do dump em partes de 4GB (caso a RAM seja superior a 16GB)
- Barra de progresso para acompanhamento
- Binário compilado estaticamente para rodar em qualquer sistema Linux

## 📦 Instalação

### 1️⃣ Clonar o repositório

```bash
git clone https://github.com/AnubisChacal/Dump_mem.git
cd Dump_mem
```

### 2️⃣ Instalar dependências

Certifique-se de ter o Rust instalado. Se ainda não tiver:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Adicione o target MUSL para compilar um binário portátil:

```bash
rustup target add x86_64-unknown-linux-musl
```

### 3️⃣ Compilar o binário

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

Isso gerará o executável em:

```
target/x86_64-unknown-linux-musl/release/Dump_mem
```

## 🎯 Uso

### 1️⃣ Executar diretamente

```bash
sudo ./target/x86_64-unknown-linux-musl/release/Dump_mem
```

### 2️⃣ Criar um pacote compactado

```bash
tar -czvf Dump_mem.tar.gz -C target/x86_64-unknown-linux-musl/release Dump_mem
```

Depois, você pode extrair e rodar em qualquer sistema Linux com:

```bash
tar -xzvf Dump_mem.tar.gz
sudo ./Dump_mem
```

## ⚠️ Aviso

- É necessário executar como **root** para acessar `/dev/mem`
- Use com responsabilidade, pois o dump de memória pode conter informações sensíveis

## 📄 Licença

Este projeto está licenciado sob a MIT License - veja o arquivo [LICENSE](LICENSE) para mais detalhes.

