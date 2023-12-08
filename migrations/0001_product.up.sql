-- Add up migration script here
CREATE TABLE product (
    id_product INT AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL,
    qty INT NOT NULL,
    price DOUBLE NOT NULL,
    description TEXT NULL,
    PRIMARY KEY (id_product)
);