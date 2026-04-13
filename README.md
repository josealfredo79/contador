# 🎯 Smart Contract Contador - Soroban

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Soroban-SDK_25.3.0-blue.svg" alt="Soroban">
  <img src="https://img.shields.io/badge/Stellar-TestNet-green.svg" alt="TestNet">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License">
</p>

Contrato inteligente de contador desarrollado en **Rust** para la plataforma **Soroban** de **Stellar**. Tutorial completo paso a paso para aprender desarrollo blockchain.

## 📋 Tabla de Contenidos

- [Descripción](#descripción)
- [Arquitectura](#arquitectura)
- [Funciones del Contrato](#funciones-del-contrato)
- [Requisitos](#requisitos)
- [Instalación](#instalación)
- [Uso](#uso)
- [Compilación](#compilación)
- [Pruebas](#pruebas)
- [Despliegue](#despliegue)
- [API Node.js](#api-nodejs)
- [Contribución](#contribución)
- [Licencia](#licencia)

## 📖 Descripción

Este proyecto es un contrato inteligente simple de tipo **Contador** que demuestra los fundamentos del desarrollo con Soroban:

- Persistencia de datos en blockchain
- Funciones de lectura y escritura
- Manejo de errores
- Testing unitario

Perfecto como punto de partida para aprender Soroban y desarrollar contratos más complejos.

## 🏗️ Arquitectura

```
┌─────────────────────────────────────────────────────────────────┐
│                    Smart Contract Contador                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────┐    ┌──────────┐    ┌───────────┐    ┌─────────┐ │
│   │  Rust  │───▶│  WASM    │───▶│  TestNet  │───▶│   API   │ │
│   │  Code  │    │  Binary  │    │  Stellar  │    │ Node.js │ │
│   └─────────┘    └──────────┘    └───────────┘    └─────────┘ │
│                                                                 │
│   src/lib.rs    compile     deploy         express.js           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## ⚙️ Funciones del Contrato

| Función | Descripción | Retorna |
|---------|-------------|---------|
| `increment()` | Incrementa el contador en 1 | `u32` |
| `decrement()` | Decrementa en 1 (mínimo 0) | `u32` |
| `get()` | Obtiene el valor actual | `u32` |
| `reset()` | Reinicia el contador a 0 | - |

## 📦 Requisitos

- **Rust** 1.70 o superior
- **Stellar CLI** (soroban-cli)
- **Node.js** 18+ (para la API)
- Target WASM: `wasm32v1-none`

## 🚀 Instalación

### 1. Clonar el repositorio

```bash
git clone https://github.com/USERNAME/soroban-contador.git
cd soroban-contador
```

### 2. Instalar Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 3. Instalar Stellar CLI

```bash
cargo install --locked soroban-cli
```

### 4. Agregar target WASM

```bash
rustup target add wasm32v1-none
```

## 📝 Uso

### Compilar el contrato

```bash
# Versión de desarrollo (más rápida)
cargo build --target wasm32v1-none

# Versión de producción (optimizada)
cargo build --target wasm32v1-none --release
```

### Ejecutar pruebas

```bash
# Todos los tests
cargo test

# Test específico
cargo test test_increment

# Con output detallado
cargo test -- --nocapture
```

### Desplegar en TestNet

```bash
# 1. Configurar red
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# 2. Generar identidad
soroban config identity generate mi-cuenta

# 3. Solicitar XLM de prueba
curl "https://friendbot.stellar.org/?addr=$(soroban config identity address mi-cuenta)"

# 4. Desplegar contrato
soroban contract deploy \
  --wasm target/wasm32v1-none/release/contador.wasm \
  --network testnet \
  --source mi-cuenta
```

### Invocar funciones

```bash
# Consultar valor
stellar contract invoke --id CONTRACT_ID --source mi-cuenta --network testnet -- get

# Incrementar
stellar contract invoke --id CONTRACT_ID --source mi-cuenta --network testnet -- increment

# Decrementar
stellar contract invoke --id CONTRACT_ID --source mi-cuenta --network testnet -- decrement

# Resetear
stellar contract invoke --id CONTRACT_ID --source mi-cuenta --network testnet -- reset
```

## 🔧 Compilación

El archivo WASM se genera en:
```
target/wasm32v1-none/release/contador.wasm
```

**Tamaño típico:** ~1 KB (optimizado)

## 🧪 Pruebas

| Test | Descripción | Estado |
|------|-------------|--------|
| `test_increment` | Incrementa correctamente | ✅ |
| `test_decrement` | Decrementa sin bajar de 0 | ✅ |
| `test_reset` | Reinicia a 0 | ✅ |
| `test_get_initial_value` | Valor inicial es 0 | ✅ |

## 🌐 API Node.js

### Instalación

```bash
cd api
npm install
cp .env.example .env
# Editar .env con tus credenciales
```

### Variables de entorno (.env)

```env
CONTRACT_ID=tu_contract_id
PUBLIC_KEY=tu_public_key
SECRET_KEY=tu_secret_key
```

### Iniciar servidor

```bash
npm start
```

### Endpoints

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/contador` | Obtener valor actual |
| POST | `/contador/increment` | Incrementar +1 |
| POST | `/contador/decrement` | Decrementar -1 |
| POST | `/contador/reset` | Reiniciar a 0 |

### Probar con cURL

```bash
curl http://localhost:3000/contador
curl -X POST http://localhost:3000/contador/increment
```

## 🔍 Verificación

Verifica tu contrato desplegado en:
- [Stellar Expert](https://stellar.expert/explorer/testnet)
- [Stellar Laboratory](https://laboratory.stellar.org)

## 📚 Recursos

- [Documentación de Stellar](https://developers.stellar.org/docs)
- [Soroban by Example](https://soroban.stellar.org/docs/examples)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Discord Stellar Developers](https://discord.gg/stellar)

## 🤝 Contribución

1. Fork el repositorio
2. Crea una rama (`git checkout -b feature/nueva-funcion`)
3. Commit tus cambios (`git commit -m 'Agregar nueva función'`)
4. Push a la rama (`git push origin feature/nueva-funcion`)
5. Abre un Pull Request

## ⚠️ Buenas Prácticas

- ✅ Validar inputs antes de modificar estado
- ✅ Usar `saturating_sub` para evitar underflow
- ✅ Escribir tests para todas las funciones
- ✅ Documentar funciones complejas
- ⚠️ Cuidado con: Reentrancy, Integer Overflow, permisos

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver el archivo [LICENSE](LICENSE) para más detalles.

---

<p align="center">
  Desarrollado con ❤️ para la comunidad Stellar
</p>
