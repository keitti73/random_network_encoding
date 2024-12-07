use rand::Rng;

fn main() {
    // サンプルデータ
    let data = vec![1, 2, 3, 4, 5];
    let mut all_encoded_data_with_sum = Vec::new();
    
    for i in 1..=5 {
        // 符号化されたデータを生成
        let (encoded_data, coefficients) = random_network_coding(&data);
        
        // 復号化されたデータを生成
        let decoded_data = random_network_decoding(&encoded_data, &coefficients);
        
        // エンコードデータの和を計算
        let encoded_sum: u32 = encoded_data.iter().map(|&x| x as u32).sum();
        
        // エンコードデータの和を追加した新しいベクターを作成
        let mut coefficients_sum = coefficients.clone();
        coefficients_sum.push(encoded_sum as u16);
        
        // エンコードデータとその和を同じ行列内に格納
        all_encoded_data_with_sum.push(coefficients_sum.clone());
        
        println!("Iteration {}:", i);
        println!("Original data: {:?}", data);
        println!("Encoded data: {:?}", encoded_data);
        println!("Coefficients: {:?}", coefficients);
        println!("Decoded data: {:?}", decoded_data);
        println!("Sum of encoded data: {:?}", encoded_sum);
        println!("Coefficients with sum: {:?}", coefficients_sum);
        println!();
    }
    
    // 最後にすべてのエンコードデータとその和を表示
    println!("All Coefficients with sum: {:?}", all_encoded_data_with_sum);
}

fn random_network_coding(data: &Vec<u8>) -> (Vec<u16>, Vec<u16>) {
    let mut rng = rand::thread_rng();
    let mut encoded_data = Vec::new();
    let mut coefficients = Vec::new();
    
    for &value in data.iter() {
        // 0から100までのランダムな係数を生成
        let coefficient: u16 = rng.gen_range(0..=100);
        // 符号化
        encoded_data.push(value as u16 ^ coefficient);
        coefficients.push(coefficient);
    }
    
    (encoded_data, coefficients)
}

fn random_network_decoding(encoded_data: &Vec<u16>, coefficients: &Vec<u16>) -> Vec<u16> {
    let mut decoded_data = Vec::new();
    
    for (encoded_value, &coefficient) in encoded_data.iter().zip(coefficients.iter()) {
        // 復号化
        decoded_data.push(encoded_value ^ coefficient);
    }
    
    decoded_data
}