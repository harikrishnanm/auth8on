-- Your SQL goes here

CREATE TABLE jwks(
    id SERIAL PRIMARY KEY,
    created TIMESTAMP NOT NULL,
    current BOOLEAN NOT NULL,
    p TEXT NOT NULL,
    q TEXT NOT NULL,
    d TEXT NOT NULL,
    qi TEXT NOT NULL,
    dp TEXT NOT NULL,
    dq TEXT NOT NULL,
    n TEXT NOT NULL,
    e TEXT NOT NULL,
    kty VARCHAR(30) NOT NULL,
    use VARCHAR(10) NOT NULL,
    alg VARCHAR(10) NOT NULL,
    kid UUID NOT NULL
)