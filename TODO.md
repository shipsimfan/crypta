# ToDo
 1. Split SHA-2 into SHA-256 (224/256) and SHA-512 (384/512)
 2. Add tool executable
 3. Add MAC framework and implement HMAC (RFC 2104)
 4. Add HKDF (RFC 5869)
 5. Add symmetric-key encryption framework and implement both AES block cipher (FIPS 197) and ChaCha20 stream cipher (RFC 7539)
 6. Add CSPRNG for block and stream ciphers
 7. Add public-key framework and implement RSA (RFC 8017) keys
   1. Add private keys
     1. Add prime number generation
     2. Add private key generation
     3. Add ASN.1 generation
     4. Add ASN.1 parsing
   2. Add public keys
     1. Add public key derivation
     2. Add ASN.1 generation
     3. Add ASN.1 parsing
 8. Add digitial signature framework and implement RSA (RFC 8017) digitial signatures
 9. Add asymmetric encryption framework and implement RSA (RFC 8017) asymmetric encryption
 10. Add Diffie-Hellman framework and implement finite field Diffie-Hellmen
 11. Implement Poly1305 MAC (RFC 7539)
 12. Add AEAD framework and implement ChaCha20-Poly1305 (RFC 7539)

# Non-core tasks
 1. Optimize SHA-1 with x86 processor instructions
 2. Optimize SHA-2 with x86 processor instructions