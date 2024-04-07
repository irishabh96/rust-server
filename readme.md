# Rust Actix Web Server

This is a simple web server containing CRUD written in Rust using the Actix framework with MongoDB.

## Folder Structure

```
/src
├── api
│   ├── mod.rs
│   └── user_api.rs
├── main.rs
├── models
│   ├── mod.rs
│   └── user_model.rs
└── repository
    ├── mod.rs
    └── mongodb_repo.rs
```

## How to Run

1. Copy the `.env.example` file to `.env`:
   ```
   cp .env.example .env
   ```
2. Ensure MongoDB is running.
3. Run the server using Cargo:
   ```
   cargo run
   ```

## APIs Included

- **GET Single Resource**: `/api/resource/{id}`
  Retrieves a single resource by its ID parameter.

- **Update Resource**: `/api/resource/{id}`
  Updates a resource with the provided ID parameter.
    - Body:
      ```
      {
          "name": "String",
          "location": "String",
          "title": "String"
      }
      ```

- **Delete Resource**: `/api/resource/{id}`
  Deletes a resource with the provided ID parameter.

- **Create Resource**: `/api/resource`
  Creates a new resource.
    - Body:
      ```
      {
          "name": "String",
          "location": "String",
          "title": "String"
      }
      ```

- **Get All Resources**: `/api/resource`
  Retrieves all resources.

Please refer to the source code for detailed implementation of these APIs.