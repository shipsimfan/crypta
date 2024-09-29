# ToDo
 1. Add no_std with `std` and `alloc` feature flags
 2. Add SHA3
 3. Add tool executable
 4. Add Hash_DRGB (NIST 800-90A)
   - Need to include lib-rand with `rand` feature flag
 5. Add MAC framework and implement HMAC (RFC 2104)
 6. Add HMAC_DRGB (NIST 800-90A)
 7. Add HKDF (RFC 5869)
 8. Add block cipher framework and implement both AES block cipher (FIPS 197)
 9. Add block cipher modes
   1. Electronic Code Book (NIST 800-38A)
   2. Cipher Block Chaining (NIST 800-38A)
   3. Cipher Feedback (NIST 800-38A)
   4. Output Feedback (NIST 800-38A)
   5. Counter (NIST 800-38A)
   6. Galois Counter and GMAC (NIST 800-38D)
 10. Add CTR_DRBG (NIST 800-90A)
 11. Add stream cipher framework and ChaCha20 stream cipher (RFC 7539)
 12. Add CSPRNG for ChaCha20
 13. Add symmetric-key encryption framework 
 14. Add public-key framework and implement RSA (RFC 8017) keys
   1. Add private keys
     1. Add prime number generation
       - Need to add lib-bigint
     2. Add private key generation
     3. Add ASN.1 generation
     4. Add ASN.1 parsing
   2. Add public keys
     1. Add public key derivation
     2. Add ASN.1 generation
     3. Add ASN.1 parsing
 15. Add digitial signature framework and implement RSA (RFC 8017) digitial signatures
 16. Add asymmetric encryption framework and implement RSA (RFC 8017) asymmetric encryption
 17. Add Diffie-Hellman framework and implement finite field Diffie-Hellmen
 18. Add Poly1305 MAC (RFC 7539)
 19. Add AEAD framework and implement ChaCha20-Poly1305 (RFC 7539)
 20. Add elliptic curve Diffie-Hellman
 21. Add ECDSA

# Non-core tasks
 1. Split SHA-2 into SHA-256 (224/256) and SHA-512 (384/512)
   1. Optimize SHA-1 with x86 processor instructions
   2. Optimize SHA-2 with x86 processor instructions