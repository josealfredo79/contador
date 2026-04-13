# Tutorial de Comandos - Smart Contracts Soroban

## 1. Instalación del Entorno

### Instalar Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Instalar Stellar CLI (Soroban)
```bash
cargo install --locked soroban-cli
```

### Verificar instalación
```bash
soroban --version
```

### Agregar target WASM
```bash
rustup target add wasm32-unknown-unknown
```

### Instalar Node.js
```bash
# Ubuntu/Debian
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verificar
node --version
npm --version
```

---

## 2. Crear y Configurar Proyecto

### Inicializar proyecto Soroban
```bash
soroban contract init mi-contrato
cd mi-contrato
```

### O crear manualmente
```bash
mkdir mi-contrato && cd mi-contrato
touch Cargo.toml src/lib.rs
```

---

## 3. Compilación

### Compilar a WASM (desarrollo)
```bash
cargo build --target wasm32-unknown-unknown
```

### Compilar a WASM (producción)
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Optimizar binario WASM
```bash
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/mi_contrato.wasm
```

### Limpiar y recompilar
```bash
cargo clean && cargo build --target wasm32-unknown-unknown --release
```

---

## 4. Pruebas

### Ejecutar todas las pruebas
```bash
cargo test
```

### Ejecutar pruebas con output detallado
```bash
cargo test -- --nocapture
```

### Ejecutar una prueba específica
```bash
cargo test nombre_del_test
```

### Ver cobertura de pruebas
```bash
cargo test -- --show-output
```

---

## 5. Configuración de Red

### Agregar red TestNet
```bash
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

### Agregar red MainNet
```bash
soroban config network add mainnet \
  --rpc-url https://soroban-rpc.stellar.org \
  --network-passphrase "Public Global Stellar Network ; June 2021"
```

### Ver redes configuradas
```bash
soroban config network list
```

---

## 6. Gestión de Identidades

### Generar nueva identidad
```bash
soroban config identity generate nombre_identidad
```

### Ver dirección de identidad
```bash
soroban config identity address nombre_identidad
```

### Listar identidades
```bash
soroban config identity list
```

### Exportar identidad a archivo
```bash
soroban config identity export nombre_identidad > clave.json
```

---

## 7. Despliegue de Contratos

### Desplegar contrato en TestNet
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/mi_contrato.wasm \
  --network testnet \
  --source nombre_identidad
```

### Desplegar con saldo específico
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/mi_contrato.wasm \
  --network testnet \
  --source nombre_identidad \
  --salt 123456789
```

---

## 8. Invocar Funciones del Contrato

### Consultar valor (lectura)
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source nombre_identidad \
  --network testnet \
  -- get
```

### Incrementar contador
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source nombre_identidad \
  --network testnet \
  -- increment
```

### Decrementar contador
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source nombre_identidad \
  --network testnet \
  -- decrement
```

### Resetear contador
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source nombre_identidad \
  --network testnet \
  -- reset
```

### Invocar con argumentos
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source nombre_identidad \
  --network testnet \
  -- increment_by 5
```

---

## 9. Exploración y Debugging

### Ver información del contrato WASM
```bash
soroban contract inspect \
  --wasm target/wasm32-unknown-unknown/release/mi_contrato.wasm
```

### Listar funciones del contrato
```bash
soroban contract info \
  --wasm target/wasm32-unknown-unknown/release/mi_contrato.wasm
```

### Simular transacción
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source nombre_identidad \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  -- increment \
  --simulate
```

---

## 10. API Node.js

### Crear proyecto API
```bash
mkdir api-contador && cd api-contador
npm init -y
npm install @stellar/stellar-sdk express dotenv
```

### Variables de entorno (.env)
```bash
CONTRACT_ID=tu_contract_id
PUBLIC_KEY=tu_public_key
SECRET_KEY=tu_secret_key
```

### Iniciar servidor
```bash
node index.js
```

### Con nodemon (desarrollo)
```bash
npm install -D nodemon
npx nodemon index.js
```

---

## 11. Endpoints de la API

### Consultar contador
```bash
curl http://localhost:3000/contador
```

### Incrementar
```bash
curl -X POST http://localhost:3000/contador/increment
```

### Decrementar
```bash
curl -X POST http://localhost:3000/contador/decrement
```

### Resetear
```bash
curl -X POST http://localhost:3000/contador/reset
```

---

## 12. Verificar Despliegue en Exploradores

### Stellar Expert (TestNet)
```
https://stellar.expert/explorer/testnet/contract/CONTRACT_ID
```

### Stellar Laboratory
```
https://laboratory.stellar.org/#account-creator
```

---

## 13. Solución de Problemas

### Error: target not found
```bash
rustup target add wasm32-unknown-unknown
```

### Error: insufficient funds
Visitar: https://laboratory.stellar.org/#account-creator

### Error: contract not found
Verificar que el CONTRACT_ID sea correcto y esté desplegado en testnet.

### Compilación lenta
Usar `cargo build --release` solo cuando sea necesario.

### Tests fallan
Revisar que las dependencias en Cargo.toml estén actualizadas.

---

## 14. Comandos Útiles Adicionales

### Ver ayuda de soroban
```bash
soroban --help
soroban contract --help
soroban config --help
```

### Ver historial de transacciones
```bash
stellar contract tx history CONTRACT_ID --network testnet
```

### Estimar costo de operación
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source nombre_identidad \
  --network testnet \
  -- estimate
```

### Fetch XLM de prueba (Faucet)
```bash
curl "https://friendbot.stellar.org/?addr=PUBLIC_KEY"
```
