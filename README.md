# 💎 Certificados de Diamantes en Solana

Proyecto desarrollado con **Anchor** sobre la red de **Solana** para gestionar el inventario y la certificación de diamantes de una joyería.

La idea principal es registrar en blockchain la información clave de cada diamante, incluyendo su **número de serie grabado con láser** y su **estado de autenticación**, con el fin de fortalecer la **trazabilidad**, la **seguridad** y la **confianza** sobre cada pieza.

Al almacenar estos datos en Solana, los registros quedan protegidos por las propiedades de la blockchain, lo que ayuda a evitar alteraciones no autorizadas y aporta una capa adicional de integridad sobre los certificados digitales.

---

## 📘 Descripción general

El programa implementa una lógica similar a un **CRUD backend** dentro de un contrato inteligente de Solana.  
Cada inventario pertenece a un propietario y puede almacenar un conjunto de diamantes certificados.

El contrato fue construido en Rust dentro del archivo `lib.rs` y utiliza las herramientas del framework **Anchor** para definir cuentas, instrucciones, validaciones y manejo de errores.

---

## 🛠️ Funcionalidades principales

El contrato inteligente incluye las siguientes instrucciones:

| Función | Descripción |
| --- | --- |
| `inicializar_inventario` | Crea e inicializa un inventario asociado al propietario de la cuenta. |
| `registrar_diamante` | Registra un nuevo diamante dentro del inventario con su número de serie, quilataje y estado de autenticación. |
| `remover_diamante` | Elimina un diamante del inventario a partir de su número de serie. |
| `alternar_autenticacion` | Cambia el estado de autenticación de un diamante específico. |
| `transferir_diamante` | Gestiona la salida de un diamante del inventario tras una venta o movimiento. |
| `ver_inventario` | Permite consultar el inventario actual mediante los logs del programa. |

---

## 🧱 Estructura de datos

### Cuenta `Inventario`

La cuenta principal del programa almacena la información general del inventario:

- `owner`: clave pública del propietario del inventario.
- `nombre_sucursal`: nombre de la sucursal asociada.
- `diamantes`: lista de diamantes registrados.

### Estructura `Diamante`

Cada diamante contiene:

- `numero_serie`: identificador único grabado en la pieza.
- `quilates`: cantidad de quilates del diamante.
- `autenticado`: estado de autenticación del certificado.

---

## 📦 Uso del almacenamiento en Solana

La cuenta `Inventario` utiliza `#[derive(InitSpace)]` para calcular el espacio requerido de forma más clara y eficiente, evitando reservar memoria innecesaria y ayudando a reducir costos de almacenamiento en la red.

### Capacidad definida

- **Propietario**: `Pubkey` de 32 bytes.
- **Nombre de sucursal**: hasta 50 caracteres.
- **Inventario de diamantes**: hasta 20 registros por cuenta.
- **Número de serie por diamante**: hasta 32 caracteres.

Esta limitación permite mantener el contrato simple y controlar mejor el costo de renta de las cuentas en Solana.

---

## 🔐 Validaciones incluidas

El programa incorpora validaciones para proteger la integridad del inventario:

- Solo el propietario del inventario puede modificarlo.
- Si un usuario no autorizado intenta realizar cambios, la operación falla.
- Si se intenta operar sobre un diamante inexistente, el programa devuelve un error controlado.

### Errores personalizados

| Error | Descripción |
| --- | --- |
| `NoAutorizado` | Se produce cuando una cuenta sin permisos intenta modificar el inventario. |
| `DiamanteNoEncontrado` | Se produce cuando no existe un diamante con el número de serie indicado. |

---

## 🧪 Flujo general de uso

1. El propietario inicializa un inventario.
2. Se registra el nombre de la sucursal.
3. Se agregan diamantes al inventario.
4. Se puede consultar el estado del inventario.
5. Se puede actualizar el estado de autenticación de una pieza.
6. Se pueden remover o transferir diamantes según sea necesario.

---

## 💻 Configuración y despliegue

### Program ID

```rust
declare_id!("5s3YhX3DyzgeMng669Hg17fEeH64iLCdSp4dn6uYW8Cf");
