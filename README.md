# Distributed Task Queue with Axum, Redis, and PostgreSQL  

This project is a distributed task queue built with Rust using:  

- Axum for the web server  
- Redis as a task queue  
- PostgreSQL for task storage  
- Tokio for async execution  

## Features  

- Task Creation - API endpoint to submit tasks  
- Redis Queue - Tasks are pushed to Redis for processing  
- Worker Pool - Multiple workers fetch and process tasks  
- Database Storage - Processed tasks are saved in PostgreSQL  
