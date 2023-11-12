#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub block_index: u64,
    pub vm_state: String,
    pub size: u32,
    pub version: u8,
    pub nonce: u64,
    pub sender: String,
    pub sysfee: String,
    pub netfee: String,
    pub valid_until: u64,
    pub signers: String,
    pub script: String,
    pub witnesses: String,
    pub stack_result: String,
    pub notifications: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub hash: String,
    pub size: u32,
    pub version: u8,
    pub merkle_root: String,
    pub time: u64,
    pub nonce: String,
    pub speaker: u8,
    pub next_consensus: String,
    pub reward: f64,
    pub reward_receiver: String,
    pub witnesses: String,
}

impl Block {
    pub fn genesis_block() -> Block {
        Block {
            hash: String::from("0x1f4d1defa46faa5e7b9b8d3f79a06bec777d7c26c4aa5f6f5899a291daa87c15"),
            size: 114,
            version: 0,
            merkle_root: String::from("0x0000000000000000000000000000000000000000000000000000000000000000"),
            time: 1468595301000,
            nonce: String::from("000000007C2BAC1D"),
            speaker: 0,
            next_consensus: String::from("NSiVJYZej4XsxG5CUpdwn7VRQk8iiiDMPM"),
            reward: 0.5,
            reward_receiver: String::from("NZeAarn3UMCqNsTymTMF2Pn6X7Yw3GhqDv"),
            witnesses: r#"[{"invocation":"DEAq7W/jUhpMon1t9muqXKfBvNyGwLfFFM1vAxrMKvUl6MqK+LL/lJAJP9uAk/cberIWWhSsdcxUtltkBLemg/VuDECQZGuvP93JlZga2ml8cnbe5cNiGgO0EMrbGYyzvgr8calP5SwMNPSYms10gIHxlsuXDU++EQpZu/vKxfHoxdC5DEDgsA3POVZdfN+i5+ekvtsaIvif42n0GC+dZi3Rp37ETmt4NtkoK2I2UXi+WIjm5yXLJsPhAvEV6cJSrvqBdsQBDEDTS6NU+kB+tgeBe9lWv+6y0L2qcUBIaUxiTCaNWZtLPghQICBvjDz1/9ttJRXG3I5N9CFDjjLKCpdIY842HW4/DEC+wlWjkCzVqzKslvpCKZbEPUGIf87CFAD88xqzl26m/TpTUcT0+D5oI2bVzAk0mcdBTPnyjcNbv17BFmr63+09","verification":"FQwhAkhv0VcCxEkKJnAxEqXMHQkj/Wl6M0Br1aHADgATsJpwDCECTHt/tsMQ/M8bozsIJRnYKWTqk4aNZ2Zi1KWa1UjfDn0MIQKq7DhHD2qtAELG6HfP2Ah9Jnaw9Rb93TYoAbm9OTY5ngwhA7IJ/U9TpxcOpERODLCmu2pTwr0BaSaYnPhfmw+6F6cMDCEDuNnVdx2PUTqghpucyNUJhkA7eMbaNokGOMPUalrc4EoMIQLKDidpe5wkj28W4IX9AGHib0TahbWO6DXBEMql7DulVAwhAt9I9g6PPgHEj/QLm38TENeosqGTGIvv4cLj33QOiVCTF0Ge0Nw6"}]"#.to_string()
        }
    }
}
