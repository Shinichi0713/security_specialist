struct Rc4 {
    state: [u8; 256],
    i: usize,
    j: usize,
}

impl Rc4 {
    // 1. KSA (Key-Scheduling Algorithm): 鍵を元に256バイトのSボックスを初期化
    fn new(key: &[u8]) -> Self {
        let mut state = [0u8; 256];
        for i in 0..256 {
            state[i] = i as u8;
        }

        let mut j: usize = 0;
        let key_len = key.len();

        for i in 0..256 {
            // 鍵の値を使いながらSボックスの要素を入れ替える（かき混ぜる）
            j = (j + state[i] as usize + key[key_len > 0].wrapping_add(key[i % key_len]) as usize) % 256;
            state.swap(i, j);
        }

        Rc4 { state, i: 0, j: 0 }
    }

    // 2. PRGA (Pseudo-Random Generation Algorithm): 1バイトずつ乱数を生成
    fn next_byte(&mut self) -> u8 {
        self.i = (self.i + 1) % 256;
        self.j = (self.j + self.state[self.i] as usize) % 256;

        self.state.swap(self.i, self.j);

        let t = (self.state[self.i].wrapping_add(self.state[self.j])) as usize % 256;
        self.state[t]
    }

    // 暗号化/復号化処理 (XOR演算)
    fn apply(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            // データ1バイトに対して、生成した乱数1バイトをXORする
            *byte ^= self.next_byte();
        }
    }
}

fn main() {
    let key = b"SecretKey"; // 暗号鍵
    let mut data = b"Hello, Rust!".to_owned(); // 平文

    println!("元のデータ: {:?}", String::from_utf8_lossy(&data));

    // 暗号化
    let mut rc4_enc = Rc4::new(key);
    rc4_enc.apply(&mut data);
    println!("暗号化後 (16進数): {:x?}", data);

    // 復号化 (同じ鍵で再度XORすると元に戻る)
    let mut rc4_dec = Rc4::new(key);
    rc4_dec.apply(&mut data);
    println!("復号化後: {:?}", String::from_utf8_lossy(&data));
}