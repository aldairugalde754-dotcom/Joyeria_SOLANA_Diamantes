# 💎 JoyeriaBlockchain - Solana Smart Contract

Este proyecto consiste en un programa desarrollado con el framework **Anchor** para la red de **Solana**, diseñado para gestionar el inventario de una joyería de lujo de forma inmutable y transparente. El sistema permite el rastreo completo de diamantes, desde su registro inicial hasta su transferencia definitiva.

---

## 🚀 Características Técnicas

* **Gestión de Inventario Basada en PDAs**: Utiliza *Program Derived Addresses* para asegurar que cada dueño tenga su propio inventario único ligado a su llave pública.
* **Seguridad y Autorización**: Implementa macros de validación `require!` para garantizar que solo el propietario original pueda modificar sus registros.
* **Trazabilidad**: Cada diamante cuenta con un número de serie único, pesaje en quilates y un estado de autenticación verificable on-chain.

---

## 🛠️ Estructura del Programa

El Smart Contract (`lib.rs`) expone las siguientes funciones principales:

| Función | Descripción |
| :--- | :--- |
| `inicializar_inventario` | Configura el espacio en la blockchain y asigna un nombre a la sucursal. |
| `registrar_diamante` | Añade una nueva pieza al stock con sus metadatos técnicos. |
| `alternar_autenticacion` | Permite actualizar el estatus de certificación de una pieza específica. |
| `transferir_diamante` | Gestiona la salida de activos del sistema tras una venta o movimiento. |
| `ver_inventario` | Función de lectura para visualizar el estado actual del stock en logs. |

---

## 📦 Requisitos de Almacenamiento

La cuenta de `Inventario` está optimizada utilizando la macro `#[derive(InitSpace)]` de Anchor para un cálculo preciso del espacio:

* **Propietario**: 32 bytes (`Pubkey`).
* **Nombre de Sucursal**: Máximo 50 caracteres (`String`).
* **Capacidad**: Vector limitado a 20 diamantes por inventario para optimizar costos de renta (Rent-Exempt).

---

## 💻 Configuración y Despliegue

### Program ID
```rust
declare_id!("5s3YhX3DyzgeMng669Hg17fEeH64iLCdSp4dn6uYW8Cf");
