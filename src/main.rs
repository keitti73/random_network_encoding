use rand::Rng;

fn main() {
    let data = vec![1, 2, 3, 4, 5]; // 元のデータ
    let mut all_encoded_data_with_sum = Vec::new(); // エンコードデータとその和を格納するベクター

    for i in 1..=5 {
        // 符号化されたデータを生成
        let (encoded_data, coefficients) = random_network_coding(&data);
        
        // 復号化されたデータを生成
        let decoded_data = random_network_decoding(&encoded_data, &coefficients);
        
        // エンコードデータの和を計算
        let encoded_sum: i32 = encoded_data.iter().fold(0, |acc, &x| acc ^ x);
        
        // エンコードデータの和を追加した新しいベクターを作成
        let mut coefficients_sum = coefficients.clone();
        coefficients_sum.push(encoded_sum as i32);
        
        // エンコードデータとその和を同じ行列内に格納
        all_encoded_data_with_sum.push(coefficients_sum.clone());
        
        // 各イテレーションの結果を表示
        println!("Iteration {}:", i);
        println!("Original data: {:?}", data); // 元のデータを表示
        println!("Encoded data: {:?}", encoded_data); // エンコードされたデータを表示
        println!("Coefficients: {:?}", coefficients); // 係数を表示
        println!("Decoded data: {:?}", decoded_data); // 復号化されたデータを表示
        println!("Sum of encoded data: {:?}", encoded_sum); // エンコードデータの和を表示
        println!("Coefficients with sum: {:?}", coefficients_sum); // 和を含む係数を表示
        println!(); // 空行を表示
    }
    
    // 最後にすべてのエンコードデータとその和を表示
    println!("All Coefficients with sum: {:?}", all_encoded_data_with_sum);
    match gaussian_elimination(&mut all_encoded_data_with_sum) {
        Some(solution) => {
            println!("解: {:?}", solution);
        }
        None => {
            println!("解なしまたは一意解なし");
        }
    }

    let solution = solve_xor_matrix(&mut all_encoded_data_with_sum);

    println!("Solution: {:?}", solution);
}

fn random_network_coding(data: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut rng = rand::thread_rng();
    let mut encoded_data = Vec::new();
    let mut coefficients:Vec<i32> = Vec::new();
    
    for &value in data.iter() {
        // 0から100までのランダムな係数を生成
        let coefficient: i32 = rng.gen_range(0..=100);
        // 符号化
        encoded_data.push(value as i32 * coefficient);
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


fn gaussian_elimination(matrix: &mut Vec<Vec<i32>>) -> Option<Vec<f64>> {
    let n = matrix.len();
    let mut mat: Vec<Vec<f64>> = matrix
        .iter()
        .map(|row| row.iter().map(|&x| x as f64).collect())
        .collect();
    
    for i in 0..n {
            // ピボット選択（最大値で列を正規化）
        let mut max_row = i;
        for k in i+1..n {
            if mat[k][i].abs() > mat[max_row][i].abs() {
                max_row = k;
            }
        }
        mat.swap(i, max_row);
    
            // 対角要素がゼロの場合は解が存在しない
        if mat[i][i] == 0.0 {
            return None;
       }
    
            // ピボット行の正規化
        for k in i+1..=n { // =nは右辺（拡張部分）も含む
            mat[i][k] /= mat[i][i];
        }
        mat[i][i] = 1.0;
    
            // 他の行を消去
        for j in 0..n {
            if j != i {
                let factor = mat[j][i];
                for k in i..=n {
                    mat[j][k] -= factor * mat[i][k];                    
                }
            }
        }

    
        // 解を取得
    }
    Some(mat.iter().map(|row| row[n]).collect())
}

fn solve_xor_matrix(matrix: &mut Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    let m = matrix[0].len() - 1; // 拡大係数行列なので最後の列が右辺

    let mut row = 0;

    for col in 0..m {
        // ピボットの選択
        let mut pivot = row;
        while pivot < n && matrix[pivot][col] == 0 {
            pivot += 1;
        }

        if pivot == n {
            continue;
        }

        // ピボット行を現在の行にスワップ
        matrix.swap(row, pivot);

        // 現在の行を正規化（他の行に影響しない形にする）
        for i in 0..n {
            if i != row && matrix[i][col] == 1 {
                for j in col..=m {
                    matrix[i][j] ^= matrix[row][j];
                }
            }
        }

        row += 1;
    }

    // 解を計算
    let mut solution = vec![0; m];
    for i in 0..n {
        let mut sum = 0;
        for j in 0..m {
            sum ^= matrix[i][j] & solution[j];
        }
        if sum != matrix[i][m] {
            solution[i] = matrix[i][m];
        }
    }

    solution
}