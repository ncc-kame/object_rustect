// json形式出力

// use pyo3::prelude::*;
// use pyo3::types::PyModule;
// use std::path::Path;

// fn main() -> PyResult<()> {
//     // Pythonインタプリタを初期化
//     pyo3::prepare_freethreaded_python();

//     Python::with_gil(|py| {
//         // sysモジュールをインポートしてパスを設定
//         let sys = py.import("sys")?;
//         let path: &pyo3::types::PyList = sys.getattr("path")?.downcast()?;
//         path.append(".")?; // カレントディレクトリをsys.pathに追加

//         // yolo.pyをインポート
//         let yolo: &PyModule = py.import("yolo")?;

//         // 絶対パスを指定
//         let image_path = Path::new(r"C:\Users\crwba\Rust_Project\object_rustect\image\dog.jpg");
//         let output_path = Path::new(r"C:\Users\crwba\Rust_Project\object_rustect\Results.json");

//         // デバッグ出力
//         println!("Image path: {}", image_path.display());
//         println!("Output path: {}", output_path.display());

//         // Pythonのdetect_objects関数を呼び出す
//         match yolo.getattr("detect_objects")?
//             .call1((image_path.to_str().unwrap(), output_path.to_str().unwrap()))
//         {
//             Ok(_) => println!("Detection complete. Results saved to: {}", output_path.display()),
//             Err(e) => eprintln!("Error during detection: {:?}", e),
//         };

//         Ok(())
//     })
// }




// 絶対パス 画像出力

// use pyo3::prelude::*;
// use pyo3::types::PyModule;
// use std::path::Path;

// fn main() -> PyResult<()> {
//     // Pythonインタプリタを初期化
//     pyo3::prepare_freethreaded_python();

//     Python::with_gil(|py| {
//         // sysモジュールをインポートしてパスを設定
//         let sys = py.import("sys")?;
//         let path: &pyo3::types::PyList = sys.getattr("path")?.downcast()?;
//         path.append(".")?; // カレントディレクトリをsys.pathに追加

//         // yolo.pyをインポート
//         let yolo: &PyModule = py.import("yolo")?;

//         // 絶対パスを指定
//         let image_path = Path::new(r"C:\Users\crwba\Rust_Project\object_rustect\image\dog.jpg");
//         let output_image_path = Path::new(r"C:\Users\crwba\Rust_Project\object_rustect\Results\detected_dog.jpg");

//         // デバッグ出力
//         println!("Image path: {}", image_path.display());
//         println!("Output image path: {}", output_image_path.display());

//         // Pythonのdetect_objects関数を呼び出す
//         match yolo.getattr("detect_objects")?
//             .call1((image_path.to_str().unwrap(), output_image_path.to_str().unwrap()))
//         {
//             Ok(_) => println!("Detection complete. Results saved to: {}", output_image_path.display()),
//             Err(e) => eprintln!("Error during detection: {:?}", e),
//         };

//         Ok(())
//     })
// }




//複数画像 可 

use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::path::Path;

fn main() -> PyResult<()> {
    // Pythonインタプリタを初期化
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        // sysモジュールをインポートしてパスを設定
        let sys = py.import("sys")?;
        let path: &pyo3::types::PyList = sys.getattr("path")?.downcast()?;
        path.append(".")?; // カレントディレクトリをsys.pathに追加

        // yolo.pyをインポート
        let yolo: &PyModule = py.import("yolo")?;

        // 入力フォルダーと出力フォルダーを指定
        let input_folder = Path::new(r"C:\Users\crwba\Rust_Project\object_rustect\image");
        let results_folder = Path::new(r"C:\Users\crwba\Rust_Project\object_rustect\Results");

        // デバッグ出力
        println!("Input folder: {}", input_folder.display());
        println!("Results folder: {}", results_folder.display());

        // Pythonのprocess_folder関数を呼び出す
        match yolo.getattr("process_folder")?
            .call1((input_folder.to_str().unwrap(), results_folder.to_str().unwrap()))
        {
            Ok(_) => println!("Batch processing complete. Results saved to: {}", results_folder.display()),
            Err(e) => eprintln!("Error during batch processing: {:?}", e),
        };

        Ok(())
    })
}
