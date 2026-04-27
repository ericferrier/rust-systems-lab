pub fn accuracy(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let mut correct = 0;

    for i in 0..y_true.len() {
        if y_true[i] == y_pred[i] {
            correct += 1;
        }
    }

    correct as f64 / y_true.len() as f64
}


pub fn confusion_matrix(y_true: &[f64], y_pred: &[f64]) -> [[usize; 10]; 10] {
    let mut matrix = [[0usize; 10]; 10];

    for i in 0..y_true.len() {
        let true_label = y_true[i] as usize;
        let pred_label = y_pred[i] as usize;

        matrix[true_label][pred_label] += 1;
    }

    matrix
}

pub fn print_confusion(matrix: &[[usize; 10]; 10]) {
    println!("\n--- Confusion Matrix ---");

    print!("    ");
    for i in 0..10 {
        print!("{:4}", i);
    }
    println!();

    for i in 0..10 {
        print!("{:2} |", i);
        for j in 0..10 {
            print!("{:4}", matrix[i][j]);
        }
        println!();
    }
}