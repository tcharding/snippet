// Package data provides data manipulation functions.
package data

// Functions here form the building blocks of the Bit Commitment
// algorithm (see Applied Cryptography - Bruce Schneier).

import (
	"crypto/aes"
	"crypto/cipher"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"math/rand"
)

const (
	// LenR is the number of bytes in 'r'.
	LenR = 32
)

func gen32bytes() []byte {
	b := make([]byte, 32)

	for i := 0; i < 4; i++ {
		r := rand.Int63()
		index := i * 8
		binary.LittleEndian.PutUint64(b[index:], uint64(r))
	}

	return b
}

// GenerateKey returns a key of length 32 bytes suitable
// for use with AES-256.
// Caller is responsible for seeding the RNG before calling.
func GenerateKey() []byte {
	return gen32bytes()
}

// GenerateR returns random bits suitable for using as 'r'
// in the Bit Commitment algorithm.
// Caller is responsible for seeding the RNG before calling.
func GenerateR() []byte {
	return gen32bytes()
}

// Commit creates commit message by encrypting (r + b).
func Commit(r string, b string, key string) (string, error) {
	// We don't need to maintain the boundary, since we
	// will verify r later, concatenation is ok here.
	payload := r + b
	return GCMEncrypter(payload, key)
}

// FromCommit is the inverse of Commit(), returns b after
// decrypting msg and verifying r.
func FromCommit(msg, r, key string) (string, error) {
	d, err := GCMDecrypter(msg, key)
	if err != nil {
		return "", fmt.Errorf("failed to decrypt card: %v", err)
	}

	gotR := d[:LenR] // 'R' from E(R,b)
	gotB := d[LenR:] // 'b' from E(R,b)

	if gotR != r {
		return "", fmt.Errorf("r in payload does not match requested")
	}
	return gotB, nil
}

// Encrypt/decrypt functions based on:
// https://gist.github.com/kkirsche/e28da6754c39d5e7ea10

// AES-GCM should be used because the operation is an authenticated encryption
// algorithm designed to provide both data authenticity (integrity) as well as
// confidentiality.

// GCMEncrypter encrypts s using k (must be 16 or 32 bytes).
func GCMEncrypter(s, k string) (string, error) {
	// Key length selects AES-128 (16 bytes) or AES-256 (32 bytes).
	key := []byte(k)
	plaintext := []byte(s)

	block, err := aes.NewCipher(key)
	if err != nil {
		return "", err
	}

	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", err
	}

	// Each key is used only once for a single message
	// so we can use a single nonce value.
	nonce := key[:12]

	ciphertext := aesgcm.Seal(nil, nonce, plaintext, nil)
	return fmt.Sprintf("%x", ciphertext), nil
}

// GCMDecrypter decrypts msg using k.
func GCMDecrypter(msg, k string) (string, error) {
	key := []byte(k) // As per GCMEncrypter.
	ciphertext, _ := hex.DecodeString(msg)

	block, err := aes.NewCipher(key)
	if err != nil {
		return "", err
	}

	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", err
	}

	nonce := key[:12]
	plaintext, err := aesgcm.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		return "", err
	}

	return string(plaintext), nil
}
