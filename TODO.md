# ToDo
 1. Add SHA3
 3. Add Hash_DRGB (NIST 800-90A)
   - Need to include lib-rand with `rand` feature flag
 4. Add MAC framework and implement HMAC (RFC 2104)
 5. Add HMAC_DRGB (NIST 800-90A)
 6. Add HKDF (RFC 5869)
 7. Add block cipher framework and implement both AES block cipher (FIPS 197)
 8. Add block cipher modes
   1. Electronic Code Book (NIST 800-38A)
   2. Cipher Block Chaining (NIST 800-38A)
   3. Cipher Feedback (NIST 800-38A)
   4. Output Feedback (NIST 800-38A)
   5. Counter (NIST 800-38A)
   6. Galois Counter and GMAC (NIST 800-38D)
 9. Add CTR_DRBG (NIST 800-90A)
 10. Add stream cipher framework and ChaCha20 stream cipher (RFC 7539)
 11. Add CSPRNG for ChaCha20
 12. Add symmetric-key encryption framework 
 13. Add public-key framework and implement RSA (RFC 8017) keys
   1. Add private keys
     1. Add prime number generation
       - Need to add lib-bigint
     2. Add private key generation
   2. Add public keys
     1. Add public key derivation
   3. Add key serializing and deserializing
     1. ASN.1
       1. PEM
       2. DER
     2. OpenSSH
 14. Add digitial signature framework and implement RSA (RFC 8017) digitial signatures
 15. Add asymmetric encryption framework and implement RSA (RFC 8017) asymmetric encryption
 16. Add Diffie-Hellman framework and implement finite field Diffie-Hellmen
 17. Add Poly1305 MAC (RFC 7539)
 18. Add AEAD framework and implement ChaCha20-Poly1305 (RFC 7539)
 19. Add elliptic curve Diffie-Hellman
 20. Add ECDSA

# Non-core tasks
 1. Add `lib-data-format` feature with `Serialize` and `Deserialize` implementations
 2. Split SHA-2 into SHA-256 (224/256) and SHA-512 (384/512)
   1. Optimize SHA-1 with x86 processor instructions
   2. Optimize SHA-2 with x86 processor instructions