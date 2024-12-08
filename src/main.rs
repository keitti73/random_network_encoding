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
        let encoded_sum: i32 = encoded_data.iter().map(|&x| x as i32).sum();
        
        // エンコードデータの和を追加した新しいベクターを作成
        let mut coefficients_sum = coefficients.clone();
        coefficients_sum.push(encoded_sum as i32);
        
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

    row_reduction (&mut all_encoded_data_with_sum);
}

fn random_network_coding(data: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut rng = rand::thread_rng();
    let mut encoded_data = Vec::new();
    let mut coefficients:Vec<i32> = Vec::new();
    
    for &value in data.iter() {
        // 0から100までのランダムな係数を生成
        let coefficient: i32 = rng.gen_range(0..=100);
        // 符号化
        encoded_data.push(value as i32 ^ coefficient);
        coefficients.push(coefficient);
    }
    
    (encoded_data, coefficients)
}

fn random_network_decoding(encoded_data: &Vec<i32>, coefficients: &Vec<i32>) -> Vec<i32> {
    let mut decoded_data = Vec::new();
    
    for (encoded_value, &coefficient) in encoded_data.iter().zip(coefficients.iter()) {
        // 復号化
        decoded_data.push(encoded_value ^ coefficient);
    }
    
    decoded_data
}

fn row_reduction(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
        // Make the diagonal contain all 1's
        let diag = matrix[i][i];
        for j in 0..cols {
            matrix[i][j] /= diag;
        }

        // Make the other rows contain 0's in the current column
        for k in 0..rows {
            if k != i {
                let factor = matrix[k][i];
                for j in 0..cols {
                    matrix[k][j] -= factor * matrix[i][j];
                }
            }
        }
    }
}