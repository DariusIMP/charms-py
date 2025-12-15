"""Type stubs for the Charms Python bindings.

Charms are tokens, NFTs or instances of arbitrary app state that sit on top of
a Bitcoin transaction. This module provides Python bindings for the core Charms
data types and operations.
"""

from typing import Any

_IntoData = Any
"""Type alias for values that can be converted to PyData.

Can be:
- PyData instance
- JSON string
- Python bytes (CBOR-encoded)
- Python int, float, bool, None
- Python list, dict, or other JSON-serializable object
"""

class PyApp:
    """Represents an application that can create, transform or destroy charms.
    
    An app is identified by a single character `tag`, a 32-byte `identity` and
    a 32-byte `vk` (verification key).
    
    The `tag` is a single character that represents the type of the app, with
    two special values:
    - `TOKEN` (tag 't') for fungible tokens
    - `NFT` (tag 'n') for non-fungible tokens (NFTs)
    
    Other values of `tag` are legal. Tokens and NFTs can be transferred without
    providing the app's implementation (RISC-V binary).
    
    The `vk` is a 32-byte hash used to verify proofs that the app's contract
    is satisfied.
    
    The `identity` is a 32-byte hash that uniquely identifies the app among
    other apps implemented using the same code.
    """
    
    tag: str
    """The single-character tag identifying the app type."""
    
    identity: "PyB32"
    """32-byte hash uniquely identifying the app."""
    
    vk: "PyB32"
    """32-byte verification key hash for proof verification."""
    
    def __init__(self, tag: str, identity: "PyB32", vk: "PyB32") -> None:
        """Create a new App.
        
        Args:
            tag: Single character tag ('t' for token, 'n' for NFT, or custom)
            identity: 32-byte hash identifying the app
            vk: 32-byte verification key hash
        """
        ...
    
    @classmethod
    def from_str(cls, s: str) -> "PyApp":
        """Create an App from a string in format 'tag_char/identity_hex/vk_hex'.
        
        Example: 't/abc123.../def456...'
        
        Args:
            s: String representation of the app
            
        Returns:
            App instance
            
        Raises:
            ValueError: If the string format is invalid
        """
        ...
    
    def __str__(self) -> str:
        """Return string representation in format 'tag_char/identity_hex/vk_hex'."""
        ...
    
    def __repr__(self) -> str:
        """Return debug representation."""
        ...

class PyB32:
    """32-byte byte string (e.g. a hash, like SHA256)."""
    
    def __init__(self, bytes: bytes) -> None:
        """Create a B32 from 32 bytes.
        
        Args:
            bytes: Exactly 32 bytes
            
        Raises:
            ValueError: If bytes length is not 32
        """
        ...
    
    @classmethod
    def from_str(cls, s: str) -> "PyB32":
        """Create a B32 from a string of 64 hex characters.
        
        Args:
            s: 64-character hex string
            
        Returns:
            B32 instance
            
        Raises:
            ValueError: If the string format is invalid
        """
        ...
    
    def __str__(self) -> str:
        """Return hex representation."""
        ...
    
    def __repr__(self) -> str:
        """Return debug representation."""
        ...
    
    def to_bytes(self) -> bytes:
        """Return the 32 bytes as a bytes object."""
        ...

class PyTxId:
    """ID (hash) of a transaction in the underlying ledger (Bitcoin).
    
    Note that string representation of transaction IDs in Bitcoin is reversed,
    and so is ours (for compatibility).
    """
    
    def __init__(self, bytes: bytes) -> None:
        """Create a TxId from 32 bytes.
        
        Args:
            bytes: Exactly 32 bytes
            
        Raises:
            ValueError: If bytes length is not 32
        """
        ...
    
    @classmethod
    def from_str(cls, s: str) -> "PyTxId":
        """Create a TxId from a string of 64 hex characters.
        
        Example: '92077a14998b31367efeec5203a00f1080facdb270cbf055f09b66ae0a273c7d'
        
        Args:
            s: 64-character hex string
            
        Returns:
            TxId instance
            
        Raises:
            ValueError: If the string format is invalid
        """
        ...
    
    def __str__(self) -> str:
        """Return hex representation (reversed for Bitcoin compatibility)."""
        ...
    
    def __repr__(self) -> str:
        """Return debug representation."""
        ...
    
    def to_bytes(self) -> bytes:
        """Return the 32 bytes as a bytes object."""
        ...

class PyUtxoId:
    """ID of a UTXO (Unspent Transaction Output) in the underlying ledger.
    
    A UTXO ID is a pair of `(transaction ID, index of the output)`.
    """
    
    txid: "PyTxId"
    """Transaction ID."""
    
    index: int
    """Index of the output within the transaction."""
    
    def __init__(self, txid: "PyTxId", index: int) -> None:
        """Create a UtxoId from a transaction ID and output index.
        
        Args:
            txid: Transaction ID
            index: Output index (0-based)
        """
        ...
    
    @classmethod
    def from_str(cls, s: str) -> "PyUtxoId":
        """Create a UtxoId from a string in format 'txid_hex:index'.
        
        Example: '92077a14998b31367efeec5203a00f1080facdb270cbf055f09b66ae0a273c7d:3'
        
        Args:
            s: String in format 'txid_hex:index'
            
        Returns:
            UtxoId instance
            
        Raises:
            ValueError: If the string format is invalid
        """
        ...
    
    def __str__(self) -> str:
        """Return string representation in format 'txid_hex:index'."""
        ...
    
    def __repr__(self) -> str:
        """Return debug representation."""
        ...
    
    def to_bytes(self) -> bytes:
        """Convert to a byte array (36 bytes).
        
        Returns:
            36-byte array: first 32 bytes are TxId, last 4 bytes are index (little-endian)
        """
        ...
    
    @classmethod
    def from_bytes(cls, bytes: bytes) -> "PyUtxoId":
        """Create UtxoId from a byte array (36 bytes).
        
        Args:
            bytes: 36-byte array
            
        Returns:
            UtxoId instance
        """
        ...

class PyData:
    """Represents a data value guaranteed to be serialized/deserialized to/from CBOR."""
    
    def __init__(self, value: _IntoData | None = None) -> None:
        """Create Data from a value.
        
        Args:
            value: Value to convert to Data. Can be:
                - None: creates empty data
                - PyData instance: returns itself
                - JSON string: parsed as JSON
                - bytes: interpreted as CBOR or UTF-8 JSON
                - int, float, bool: converted to JSON number/boolean
                - list, dict, or other JSON-serializable object
                
        Raises:
            ValueError: If the value cannot be converted to Data
        """
        ...
    
    @classmethod
    def from_json(cls, json_str: str) -> "PyData":
        """Create Data from a JSON string.
        
        The JSON will be converted to CBOR internally.
        
        Args:
            json_str: JSON representation of the data
            
        Returns:
            Data instance
            
        Raises:
            ValueError: If the JSON is invalid
        """
        ...
    
    def is_empty(self) -> bool:
        """Check if the data value is empty (null).
        
        Returns:
            True if data is empty/null
        """
        ...
    
    def bytes(self) -> bytes:
        """Serialize to bytes using CBOR encoding.
        
        Returns:
            CBOR-encoded bytes
        """
        ...
    
    def to_json(self) -> str:
        """Convert the data to a JSON string.
        
        The CBOR data will be deserialized to JSON.
        
        Returns:
            JSON string representation
            
        Raises:
            ValueError: If the data cannot be converted to JSON
        """
        ...
    
    def __repr__(self) -> str:
        """Return debug representation."""
        ...

class PyTransaction:
    """Represents a transaction involving Charms.
    
    A Charms transaction sits on top of a Bitcoin transaction. It transforms a set of
    input UTXOs into a set of output UTXOs.
    
    A Charms transaction may also reference other valid UTXOs that are not being
    spent or created.
    """
    
    def __init__(self) -> None:
        """Note: Use `from_json()` to create a Transaction, not the constructor."""
        ...
    
    @classmethod
    def from_json(cls, json_str: str) -> "PyTransaction":
        """Create a Transaction from a JSON string.
        
        Args:
            json_str: JSON representation of the transaction
            
        Returns:
            Transaction instance
            
        Raises:
            ValueError: If the JSON is invalid
        """
        ...
    
    def to_json(self) -> str:
        """Serialize the transaction to a JSON string.
        
        Returns:
            JSON string representation
        """
        ...
    
    def is_simple_transfer(self, app: PyApp) -> bool:
        """Check if the transaction is a simple transfer of assets specified by `app`.
        
        For tokens (tag 't'), this checks if token amounts are balanced.
        For NFTs (tag 'n'), this checks if NFT states are preserved.
        For other apps, returns False.
        
        Args:
            app: The app to check
            
        Returns:
            True if the transaction is a simple transfer for this app
        """
        ...
    
    def token_amounts_balanced(self, app: PyApp) -> bool:
        """Check if the provided app's token amounts are balanced in the transaction.
        
        This means that the sum of the token amounts in the transaction inputs
        is equal to the sum of the token amounts in the transaction outputs.
        
        Args:
            app: The token app to check (must have tag 't')
            
        Returns:
            True if token amounts are balanced
        """
        ...
    
    def nft_state_preserved(self, app: PyApp) -> bool:
        """Check if the NFT states are preserved in the transaction.
        
        This means that the NFTs (created by the provided `app`) in the transaction
        inputs are the same as the NFTs in the transaction outputs.
        
        Args:
            app: The NFT app to check (must have tag 'n')
            
        Returns:
            True if NFT states are preserved
        """
        ...

# Constants
TOKEN: str
"""Special `App.tag` value for fungible tokens."""

NFT: str
"""Special `App.tag` value for non-fungible tokens (NFTs)."""

# Module functions
def is_simple_transfer(app: PyApp, tx: PyTransaction) -> bool:
    """Check if the transaction is a simple transfer of assets specified by `app`.
    
    For tokens (tag 't'), this checks if token amounts are balanced.
    For NFTs (tag 'n'), this checks if NFT states are preserved.
    For other apps, returns False.
    
    Args:
        app: The app to check
        tx: The transaction to check
        
    Returns:
        True if the transaction is a simple transfer for this app
    """
    ...

def token_amounts_balanced(app: PyApp, tx: PyTransaction) -> bool:
    """Check if the provided app's token amounts are balanced in the transaction.
    
    This means that the sum of the token amounts in the transaction inputs
    is equal to the sum of the token amounts in the transaction outputs.
    
    Args:
        app: The token app to check (must have tag 't')
        tx: The transaction to check
        
    Returns:
        True if token amounts are balanced
    """
    ...

def nft_state_preserved(app: PyApp, tx: PyTransaction) -> bool:
    """Check if the NFT states are preserved in the transaction.
    
    This means that the NFTs (created by the provided `app`) in the transaction
    inputs are the same as the NFTs in the transaction outputs.
    
    Args:
        app: The NFT app to check (must have tag 'n')
        tx: The transaction to check
        
    Returns:
        True if NFT states are preserved
    """
    ...
