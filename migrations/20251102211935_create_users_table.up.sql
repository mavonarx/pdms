CREATE TABLE USERS ( 
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE,
    first_name VARCHAR(100),        
    last_name VARCHAR(100),
    role VARCHAR(20),
    created_at TIMESTAMP DEFAULT NOW()
);
