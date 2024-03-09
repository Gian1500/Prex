# Mini Procesador de Pagos en Rust

Este es un mini procesador de pagos desarrollado en Rust utilizando el framework Actix-web. El objetivo de este proyecto es proporcionar una API REST para llevar un registro del saldo de los clientes y realizar operaciones de crédito y débito.

## Requisitos

- Rust (preferiblemente la última versión estable)
- Cargo (incluido con Rust)
- Actix-web (dependencia gestionada por Cargo)

## Instalación y Ejecución

1. Clona este repositorio:
git clone https://github.com/Gian1500/Prex.git


2. Navega al directorio del proyecto

3.Compila y ejecuta el proyecto con el comando "Cargo run"


Esto iniciará el servidor Actix-web en `http://localhost:8080`.

## Uso

Una vez que el servidor esté en funcionamiento, puedes interactuar con la API REST utilizando herramientas como cURL, Postman o escribiendo scripts en Rust. Aquí tienes una descripción de los endpoints disponibles:

- `POST /new_client`: Crea un nuevo cliente con la información proporcionada.
- `POST /new_credit_transaction`: Realiza una transacción de crédito para un cliente existente.
- `POST /store_balances`: Persiste los saldos de los clientes en un archivo.
- `GET /client_balance/{client_id}`: Obtiene el saldo de un cliente por su ID.

Dentro del proyecto esta incluido el JSON en Postman para importar y probar las Request.


