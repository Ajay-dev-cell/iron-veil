#!/bin/bash
set -e

# Directory to store certificates
CERT_DIR="certs"
mkdir -p $CERT_DIR

# 1. Generate a private key for the Custom CA
echo "Generating CA key..."
openssl genrsa -out $CERT_DIR/ca.key 2048

# 2. Generate the CA certificate (valid for 10 years)
echo "Generating CA certificate..."
openssl req -x509 -new -nodes -key $CERT_DIR/ca.key \
  -sha256 -days 3650 -out $CERT_DIR/ca.crt \
  -subj "/C=US/ST=State/L=City/O=IronVeil/OU=Security/CN=IronVeilRootCA"

# 3. Generate a private key for the Server
echo "Generating Server key..."
openssl genrsa -out $CERT_DIR/server.key 2048

# 4. Create a Certificate Signing Request (CSR) configuration
cat > $CERT_DIR/csr.conf <<EOF
[ req ]
default_bits = 2048
prompt = no
default_md = sha256
req_extensions = req_ext
distinguished_name = dn

[ dn ]
C = US
ST = State
L = City
O = IronVeil
OU = Proxy
CN = localhost

[ req_ext ]
subjectAltName = @alt_names

[ alt_names ]
DNS.1 = localhost
DNS.2 = iron-veil
DNS.3 = postgres
IP.1 = 127.0.0.1
IP.2 = 0.0.0.0
EOF

# 5. Generate the Certificate Signing Request (CSR)
echo "Generating Server CSR..."
openssl req -new -key $CERT_DIR/server.key -out $CERT_DIR/server.csr -config $CERT_DIR/csr.conf

# 6. Generate the Server Certificate signed by the CA (valid for 1 year)
echo "Generating Server Certificate..."
openssl x509 -req -in $CERT_DIR/server.csr -CA $CERT_DIR/ca.crt -CAkey $CERT_DIR/ca.key \
  -CAcreateserial -out $CERT_DIR/server.crt -days 365 -sha256 -extfile $CERT_DIR/csr.conf -extensions req_ext

# Cleanup
rm $CERT_DIR/server.csr $CERT_DIR/csr.conf

echo "------------------------------------------------"
echo "Certificates generated in '$CERT_DIR/'"
echo "------------------------------------------------"
echo "1. server.key: Private key (Keep secure!)"
echo "2. server.crt: Public certificate (Configure in proxy.yaml)"
echo "3. ca.crt:     Root CA (Distribute to clients to trust the connection)"
