# Las Machas Cloud - Inventario en Solana

Este es un contrato inteligente (Smart Contract) desarrollado en Rust utilizando el framework Anchor para la red de prueba (Devnet) de Solana. 

El proyecto consiste en un sistema de inventario descentralizado (CRUD) diseñado a la medida para gestionar los lotes de salsas de UN negocio llamado "Las Machas".

## Funcionalidades (CRUD)
El programa utiliza una cuenta PDA (Program Derived Address) única por usuario para almacenar los datos de forma segura. Contiene las siguientes instrucciones:

* **`inicializar_inventario` (Create):** Crea la cuenta PDA vinculada a la billetera del usuario y le asigna un nombre a la bodega/inventario.
* **`agregar_salsa` (Update):** Añade un nuevo lote de salsa al vector del inventario, guardando el Nombre, Nivel de Picor y Cantidad en Stock.
* **`ver_inventario` (Read):** Imprime en los logs de la transacción la lista completa de las salsas registradas y sus detalles.

## Despliegue
Este programa fue compilado y desplegado exitosamente en la Devnet de Solana mediante Solana Playground.
