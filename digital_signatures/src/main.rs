use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::pkcs8::Document;
use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, KeyPair, UnparsedPublicKey, ECDSA_P256_SHA256_ASN1, ECDSA_P256_SHA256_ASN1_SIGNING};

fn main() -> Result<(), Unspecified> {

    /*
        let pair = (1, 2); // tuple of size 2 (pair)
        let single = (1); //tuple of size 1
        let empty = (); // empty*/

    // Step 1: Create a cryptographically secure random number generator
    // cryptographically-secure means that the number coming out of this generator
    // or so random that no one would ever be able to predict or even generate this
    // number ever again.
    let rand = SystemRandom::new();


    // Step 2: Use the ring library to use the random number to create a secret key
    //  The term "pkcs8" is just a format for bytes, like hex is format for bytes.
    //  We don't need to know how the format works, we just need to be able to convert
    //  it to another format we can use.
    //
    // ECDSA_P256_SHA256_ASN1_SIGNING refers to a specific type of digital signature algorithm.
    //  Don't need to know how the algorithm works, just need to know that it has a type.
    //
    // Note how we are passing the random number generator to the secret key generation function.

    let pkcs8_bytes: Document = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &rand)?;



    // Step 3: Convert the secret key which is in pkcs8 into a different format that we can
    //  use to sign messages.

    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, pkcs8_bytes.as_ref(), &rand)?;

    // Format the public key as a hex string for display
    let public_key_hex = HEXUPPER.encode( key_pair.public_key().as_ref());
    println!("Public Key: {}", public_key_hex);


    // Step 4: Take our message we want to sign and convert it an array of bytes (byte array)
    //  This message could also be a 100GB PDF that is a contract for a bank.
    const MESSAGE: &[u8]  = b"hello, world";

    // Step 5: Call the sign function using the key pair we generated.
    let sig1 = key_pair.sign(&rand, MESSAGE)?;

    // Step 5: Call the sign function using the key pair we generated.
    let sig2 = key_pair.sign(&rand, MESSAGE)?;

    // Format the signature key as a hex string for display
    let signature_hex1 = HEXUPPER.encode( sig1.as_ref());
    println!("Signature 1: {}", signature_hex1);

    // Format the signature key as a hex string for display
    let signature_hex2 = HEXUPPER.encode( sig2.as_ref());
    println!("Signature 2: {}", signature_hex2);

    // Step 8: verify the signature using the public key and message
    let public_key = UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, key_pair.public_key().as_ref());

    public_key.verify(MESSAGE, sig1.as_ref())?;

    println!("Successfully verified signature 1");

    public_key.verify(MESSAGE, sig2.as_ref())?;

    println!("Successfully verified signature 2");

    Ok(())
}
