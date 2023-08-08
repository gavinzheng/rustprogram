use std::process::Command;
use web3::types::U256;
//use hex_literal::hex;

// os.system("zokrates compile -i " + filename)
// os.system("zokrates setup")
// os.system("zokrates export-verifier")
// ZoKrates.compile("verifybid.code")
// ZoKrates.setup()
// ZoKrates.verify()

// # Simulates publishing into Blockchain or similar
// os.system("cp proving.key out.code out ../prover/")
// os.system("nc -lvp 9090")

// ef witness(R, A, S, secret, bid1, bid2, bid3):
//     os.system("zokrates compute-witness -a " + R[0] + " " + R[1] + " " + R[2] + " " + R[3] + " " + R[4] + " " + R[5] + " "  + A[0] + " "  + A[1] + " "  + A[2] + " "  + A[3] + " "  + A[4] + " "  + A[5] + " " + S[0] + " " + S[1] + " " + S[2] + " " + secret + " " + bid1 + " " + bid2 + " " + bid3)
//     return

// def proof():
//     os.system("nohup zokrates generate-proof >/dev/null 2>&1")
//     badProof = os.popen("zokrates print-proof --format remix --proofpath proof.json").read()
//     goodProof = "[" + badProof[badProof.find("[")+1:len(badProof)-65] 
//     return goodProof

//     print(arguments)
//     subprocess.call(["zokrates", "compute-witness", "-a", *arguments])

fn u256_to_u32_8(val: U256 ) -> [u32; 8]{
    let mut buf = [0u8; 32];
    let mut retu32 = [0u32; 8];
    val.to_big_endian(&mut buf);
    
    for i in 0..8 {
        let mut key_index_buf = [0u8; 4];
        key_index_buf.copy_from_slice(&buf[4*i..4*(i+1)]);
        let raw_index = u32::from_be_bytes(key_index_buf);

        retu32[i]=raw_index;
    }

    retu32
}

// Refer to withdraw.zok
// 
// Parameters
//    field[2] publicKeyA, 
//    private field secretKeyA, 
//    u32[8] rootDigest, 
//    private u32[8] leafDigest, 
//    u32[8] PathDigest0, 
//    private u32[8] PathDigest1, 
//    private bool[2] directionSelector) -> bool
//
// Example 
//    zokrates compute-witness -a 17559621506966228669946200908540191909304602050620774623186985148217036891086 3499692685359225014642568034871893823673913644950320594232804843053171036306 37232355431795428394424168963998030788617641514587324629047050974144141575298  2750958632 3860495517 2694795054 1180789744 1942258783 1698713166 1678052419 30359419 3280663902 2870896982 2870967870 472011007 359726217 4254639460 1845398842 1208882838 0 0 0 0 0 0 0 0 4121296194 3513393200 664334190 3540621211 1124089123 551153896 3935842729 660208459 0 0
//
pub fn zokcmd_computewitness(
    pk1: U256, pk2 : U256,
    sk:  U256,
    root : U256, leaf : U256, path0 : U256, path1 : U256,  direction0: U256, direction1: U256
) -> std::io::Result<()>{
    let rootdigest = u256_to_u32_8(root);
    let leafdigest = u256_to_u32_8(leaf);
    let pathdigest0 = u256_to_u32_8(path0);
    let pathdigest1 = u256_to_u32_8(path1);

    let parameters = format!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", 
                        pk1, pk2, sk, 
                        rootdigest[0], rootdigest[1],rootdigest[2],rootdigest[3],rootdigest[4],rootdigest[5],rootdigest[6],rootdigest[7],
                        leafdigest[0], leafdigest[1],leafdigest[2],leafdigest[3],leafdigest[4],leafdigest[5],leafdigest[6],leafdigest[7],
                        pathdigest0[0], pathdigest0[1],pathdigest0[2],pathdigest0[3],pathdigest0[4],pathdigest0[5],pathdigest0[6],pathdigest0[7],
                        pathdigest1[0], pathdigest1[1],pathdigest1[2],pathdigest1[3],pathdigest1[4],pathdigest1[5],pathdigest1[6],pathdigest1[7],
                        direction0, direction1);
    debug!("zokcmd_computewitness : parameters = {}", parameters);                    
    
    let cmd_str = format!("cd notes; zokrates compute-witness -a  {}; zokrates generate-proof;", parameters );
    //let cmd_str = format!("zokrates compute-witness -a  {}", parameters);
    debug!( "zokcmd_computewitness : cmd_str = {}", cmd_str); 

    let output  = Command::new("sh").arg("-c").arg(cmd_str).output().expect("sh exec error!");
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
        // zokcmd_generateproof()?;
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }

    // let output = Command::new("zokrates").arg("compute-witness").arg("-a").arg(parameters).output()?;
    // // String::from_utf8(output.stdout).unwrap()
    // //     .lines()
    // //     // .filter_map(|line| pattern.captures(line))
    // //     // .map(|cap| {
    // //     //         Commit {
    // //     //             hash: cap[1].to_string(),
    // //     //             message: cap[2].trim().to_string(),
    // //     //         }
    // //     //     })
    // //     .take(5)
    // //     .for_each(|x| println!("zokcmd_computewitness : {:?}", x));
    // if !output.status.success() {
    //     debug!("Command executed with failing error code");
    // }
    // if output.status.success() {
    //     let s = String::from_utf8_lossy(&output.stdout);

    //     print!("rustc succeeded and stdout was:\n{}", s);
    // } else {
    //     let s = String::from_utf8_lossy(&output.stderr);

    //     print!("rustc failed and stderr was:\n{}", s);
    // }

    //debug!("zokcmd_computewitness : output ={}", output.stdout); 
    Ok(())
}    

pub fn zokcmd_renameproof(
    noteid: U256
) -> std::io::Result<()>{
    let cmd_str = format!("cd notes; mv proof.json {}-proof.json", noteid.to_string() );
    debug!( "zokcmd_renameproof : cmd_str = {}", cmd_str); 

    let output  = Command::new("sh").arg("-c").arg(cmd_str).output().expect("sh exec error!");
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
        // zokcmd_generateproof()?;
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }

    Ok(())
}    


// pub fn zokcmd_setup()->std::io::Result<()>{
//     // ("zokrates setup")
//     let output = Command::new("zokrates").arg("setup").output()?;
//     if !output.status.success() {
//         debug!("Command zokrates setup executed with failing error code");
//     }

//     Ok(())
// }

pub fn zokcmd_generateproof() -> std::io::Result<()>{
    let output = Command::new("cd notes; zokrates").arg("generate-proof").output()?;
    if !output.status.success() {
        debug!("Command zokrates generete-proof executed with failing error code");
    }

    Ok(())
}    

// let output = Command::new("git").arg("log").arg("--oneline").output()?;

// if !output.status.success() {
//     error_chain::bail!("Command executed with failing error code");
// }

// let pattern = Regex::new(r"(?x)
//                            ([0-9a-fA-F]+) # commit hash
//                            (.*)           # The commit message")?;

// String::from_utf8(output.stdout)?
//     .lines()
//     .filter_map(|line| pattern.captures(line))
//     .map(|cap| {
//              Commit {
//                  hash: cap[1].to_string(),
//                  message: cap[2].trim().to_string(),
//              }
//          })
//     .take(5)
//     .for_each(|x| println!("{:?}", x));

// Ok(())

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zokcomputewitness() {
        // Example 
        // 
        // Input: python3 terminaldecimal.py a3f84c28e61a7c9da09f4f2e46616bf073c4845f65404e4e64050c4301cf3f7b c38af55eab1e6956ab1f7e3e1c2250ff1570fc89fd98a5646dfe8d3a480e1696 0000000000000000000000000000000000000000000000000000000000000000 f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b
        // 
        // Output: zokrates compute-witness -a 17559621506966228669946200908540191909304602050620774623186985148217036891086 3499692685359225014642568034871893823673913644950320594232804843053171036306 37232355431795428394424168963998030788617641514587324629047050974144141575298  2750958632 3860495517 2694795054 1180789744 1942258783 1698713166 1678052419 30359419 3280663902 2870896982 2870967870 472011007 359726217 4254639460 1845398842 1208882838 0 0 0 0 0 0 0 0 4121296194 3513393200 664334190 3540621211 1124089123 551153896 3935842729 660208459 0 0

        let witness = zokcmd_computewitness(
            U256::from_dec_str("17559621506966228669946200908540191909304602050620774623186985148217036891086").unwrap(),
            U256::from_dec_str("3499692685359225014642568034871893823673913644950320594232804843053171036306").unwrap(),
            U256::from_dec_str("37232355431795428394424168963998030788617641514587324629047050974144141575298").unwrap(),
            U256::from_big_endian(&hex!("a3f84c28e61a7c9da09f4f2e46616bf073c4845f65404e4e64050c4301cf3f7b")),
            U256::from_big_endian(&hex!("c38af55eab1e6956ab1f7e3e1c2250ff1570fc89fd98a5646dfe8d3a480e1696")),
            U256::from_big_endian(&hex!("0000000000000000000000000000000000000000000000000000000000000000")),
            U256::from_big_endian(&hex!("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b")),
            // U256::FromStr("0xc38af55eab1e6956ab1f7e3e1c2250ff1570fc89fd98a5646dfe8d3a480e1696").unwrap(),
            // U256::FromStr("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            // U256::FromStr("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b").unwrap(),
            U256::from_dec_str("1").unwrap(),
            U256::from_dec_str("0").unwrap(),
        );

    }
}
