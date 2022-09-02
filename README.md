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

经讨论确定以第一种方式进行签名。

- [ ] ToDoList 
    - [ ] complete simchain-server
    - [ ] format schnorr

