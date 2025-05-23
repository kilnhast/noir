# Function: ecdsa\_secp256k1\_verify()

```ts
function ecdsa_secp256k1_verify(
   hashed_msg, 
   public_key_x_bytes, 
   public_key_y_bytes, 
   signature): boolean
```

Verifies a ECDSA signature over the secp256k1 curve.

## Parameters

| Parameter | Type |
| ------ | ------ |
| `hashed_msg` | `Uint8Array` |
| `public_key_x_bytes` | `Uint8Array` |
| `public_key_y_bytes` | `Uint8Array` |
| `signature` | `Uint8Array` |

## Returns

`boolean`
