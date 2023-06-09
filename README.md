# bank_architecture
# UI (Angular):
The UI is built using Angular, a popular front-end web application framework.
It provides a user-friendly interface for customers to access their accounts, make deposits and withdrawals, 
view their transaction history, and receive notifications. The UI communicates with the API Gateway to access backend services and functionality.

# API Gateway:
The API Gateway serves as a single entry point for all incoming client requests. 
It routes requests to the appropriate microservices and handles authentication, authorization, and rate limiting. 
It provides a unified API for the UI to communicate with all other services.

# Load Balancer:
The Load Balancer distributes incoming client requests across multiple instances of each microservice. 
This helps to ensure high availability, fault tolerance, and optimal resource utilization. 
It monitors the health of each instance and removes any that are unresponsive or overloaded.

# Registry and Discovery:
Registry and Discovery services enable microservices to locate and communicate with one another. 
Each microservice registers its location with the discovery service upon startup. 
When a service needs to communicate with another, it queries the discovery service for its location.

# Security Service:
The Security Service handles authentication and authorization for the entire system. 
It uses OAuth2 or JWT to secure access to microservices, ensuring that only authorized users can perform specific actions.

# Account Service:
The Account Service manages customer account information, such as balances, personal details, and account status. 
It provides APIs for account creation, retrieval, and updates.

# Deposit Service:
The Deposit Service handles deposit transactions. 
It includes a circuit breaker pattern to ensure system stability and rollback operations in case of failure. 
It is integrated with the Account Service to update account balances when a deposit is made.

# Withdrawal Service:
The Withdrawal Service handles withdrawal transactions. 
It also uses a circuit breaker pattern and rollback operations for system stability. 
It communicates with the Account Service to update account balances upon successful withdrawals. Additionally, it triggers an event in RabbitMQ to store historical transaction information and send notifications.

# Historical Service:
The Historical Service stores and retrieves transaction history for each account. 
It receives events from RabbitMQ triggered by the Withdrawal Service and maintains a record of all transactions, which can be queried by the UI.

# Cache Service:
The Cache Service improves performance by storing frequently accessed data, such as account balances and transaction history, in a distributed cache. 
This reduces the need for repeated calls to the underlying services, resulting in faster response times.

# Distributed Tracing:
Distributed Tracing helps monitor and troubleshoot the system by tracking requests as they pass through multiple microservices. 
It provides visibility into the performance and health of each service, making it easier to identify and resolve issues.

# Log Aggregation:
Log Aggregation collects and stores logs from all microservices in a centralized location. 
This enables easier analysis, monitoring, and debugging of the system.

# Notification Service:
The Notification Service sends alerts and updates to customers based on specific events, such as successful withdrawals. 
It receives events from RabbitMQ, processes them, and sends notifications through various channels, such as email, SMS, or push notifications.