// use schnorr::context::SigningTranscript;
use chain_demo::sign::{PublicKey,Keypair,Signature, signing_context,verify_batch_bos,sign_aggregate,verify_batch};
use curve25519_dalek::ristretto::CompressedRistretto;
use rand_core::OsRng;
//use curve25519_dalek::scalar::Scalar;
//use curve25519_dalek::ristretto::{RistrettoPoint,CompressedRistretto};



#[allow(non_snake_case)]
fn main(){
    //公私钥产生
    //keypair为公私钥对，OsRng为随机数，keypair.secret为私钥, keypair.public为公钥
    //公私钥关系为 X=x*G    x---私钥  X---公钥 G---椭圆曲线基点
    //keypair.secret分为{key,nonce}; key为Scalar格式，Scalar为长度为32的u8数组，用于生成公钥, nonce为随机数，长度为32的u8数组
    //keypair.public 为RistrettoBoth结构 
    let keypair: Keypair = Keypair::generate_with(OsRng);

    //签名过程，按照消息不同修改message,signatures为签名结果{R,s},
    let ctx = signing_context(b"");
    let messages: [&[u8]; 7] = [
        b"Watch closely everyone, I'm going to show you how to kill a god.",
        b"I'm not a cryptographer I just encrypt a lot.",
        b"Still not a cryptographer.",
        b"This is a test of the tsunami alert system. This is only a test.",
        b"Fuck dumbin' it down, spit ice, skip jewellery: Molotov cocktails on me like accessories.",
        b"Hey, I never cared about your bucks, so if I run up with a mask on, probably got a gas can too.",
        b"And I'm not here to fill 'er up. Nope, we came to riot, here to incite, we don't want any of your stuff.", ];
    let mut signatures: Vec<Signature> = Vec::new();
    // let mut keypairs: Vec<Keypair> = Vec::new();
    
    //compressed_ristretto为存放在区块头的公钥部分
    let mut compressed_ristretto: Vec<CompressedRistretto> = Vec::new();
    //单个数据签名和验证也可仿照keypair.sign与keypair.public.verify
    for i in 0..messages.len() {
       let keypair1 = keypair.clone();
       let compressed_ristretto1=keypair1.public.as_compressed();
       let signature = keypair1.sign(ctx.bytes(messages[i]));
       if keypair.public.verify(ctx.bytes(messages[i]), &signature).is_ok(){
        println!("verify passed!");
       }
        signatures.push(signature);
        // keypairs.push(keypair1);
        compressed_ristretto.push(*compressed_ristretto1);
    }

    //根据compressed_ristretto还原整个公钥
    let public_keys: Vec<PublicKey> = compressed_ristretto.iter().map(|cr| PublicKey::recover(*cr)).collect();
    println!("{:?}\n",public_keys);
    // if PublicKey::recover(*keypair.public.as_compressed()).eq(&keypair.public){
    //     println!("recover successed");
    //     println!("{:?}\n",PublicKey::recover(*keypair.public.as_compressed()));
    // }
    // println!{"{:?}\n",&signatures};
    //批量验签
    // let public_keys: Vec<PublicKey> = keypairs.iter().map(|key| key.public).collect();
    let transcripts = messages.iter().map(|m| ctx.bytes(m));
    if verify_batch_bos(transcripts, &signatures[..], &public_keys[..], false).is_ok(){
        println!("bos_batch varify passed!");
    }
    let transcripts = messages.iter().map(|m| ctx.bytes(m));
    if verify_batch(transcripts, &signatures[..], &public_keys[..], false).is_ok(){
        println!("batch varify passed!");
    }

    //聚合签名，全节点调用sign_aggregate将多个签名合并为{s,r,rsum}形式 客户端使用得到的{s,r,rsum}及区块头中的公钥调用verify_aggregate进行验证
    let transcripts = messages.iter().map(|m| ctx.bytes(m));
    let aggre_sign=sign_aggregate(&signatures[..]);
    println!("aggregate_sign: {:?}",aggre_sign);
    if keypair.public.verify_aggregate(transcripts,aggre_sign.bs, &aggre_sign.r[..], aggre_sign.rsum).is_ok(){
        println!("aggregate_varify passed!");
    }

}