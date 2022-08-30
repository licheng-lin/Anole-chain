Input format

```
block_id [address] {in/out, amount, timestamp}
```

如何对交易进行签名？目前RawTransaction 的结构体如下。

1. 将所有数据进行字符串拼接进行签名
2. 将数据结构转为Serializable，然后对字符串进行签名

```
pub struct RawTransaction {
    pub block_id: IdType,
    pub key: KeyType,
    pub value: TransactionValue,
}
```

一个RSA应用实例的参考

```
let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    
    println!("public_key: {:?}",pub_key);
    println!("private_key: {:?}",priv_key);

    //encrypt
    let data = b"hello Coderlin007!";
    let encrypt_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..]).expect("failed to encrypt");
    println!("encrypted: {:?}", String::from(String::from_utf8_lossy(&encrypt_data)));

    //decrypt
    let decrypt_data = priv_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &encrypt_data).expect("Failed to decrypt");
    println!("decrypted: {:?}", String::from_utf8_lossy(&decrypt_data));
```

结果

```
encrypted: "�2����&8c�(�\u{1b}D똛y���8�_)$�Y�{�w��\u{19}�\u{302}e\n�<\u{17}/\u{368}\u{b}G�aԼ���\u{13}�\u{e}%T\t��b� �9B\u{16}�O�W�\u{2}*���\u{3}�Dg\u{1d}��7���r��I\u{11}\u{4}�eC�_\u{10}�\0�`A�\u{15}XѝK=\u{f}\u{1a}���_��aA\u{c}�v�`\u{b}ӝfp���L\u{14}ɢ'�HM�:�G\u{19}�ж�`po*��\u{11}c��,�\u{6}���SW���O\r�\u{1c}\u{6}C�ԍc���\u{16}A\u{6}\u{33f}0뙃�r?R�M�)���q��+���\u{7f}���5\u{1e}\u{c}��}�a��\u{19}ֈz�s��˷�P��������/Q\u{1b}"
decrypted: "hello Coderlin007!"
```

