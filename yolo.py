##json形式のコード

# from ultralytics import YOLO
# import json
# import os

# def detect_objects(image_path, output_path):

#     #ファイルパスの確認
#     if not os.path.exists(image_path):
#         raise FileNotFoundError(f"The specified image file does not exist: {image_path}")
#     print((f"Image file found: {image_path}"))

#     # YOLOモデルをロード
#     model = YOLO("yolov8n.pt")  # 小型モデル (yolov8n)
    
#     # 推論を実行
#     results = model(image_path)
    
#     # 結果を辞書形式で整形
#     detections = []
#     for box in results[0].boxes:
#         detections.append({
#             "class": int(box.cls),
#             "confidence": float(box.conf),
#             "coordinates": [float(c) for c in box.xyxy[0].tolist()]
#         })
    
#     # 結果をJSONファイルに保存
#     with open(output_path, "w") as f:
#         json.dump(detections, f)

#         print(f"Detection complete. Results saved to: {output_path}")

# # サンプル実行
# detect_objects(r"C:\Users\crwba\Rust_Project\object_rustect\image\inu.jpg",
#                r"C:\Users\crwba\Rust_Project\object_rustect\results.json")




## 絶対パス 画像形式のコード

# from ultralytics import YOLO
# import os
# import cv2

# def detect_objects(image_path, output_image_path):
#     # ファイルパスの確認
#     if not os.path.exists(image_path):
#         raise FileNotFoundError(f"The specified image file does not exist: {image_path}")
#     print(f"Image file found: {image_path}")

#     # YOLOモデルをロード
#     model = YOLO("yolov8n.pt")  # 小型モデル (yolov8n)

#     # 推論を実行
#     results = model(image_path)

#     # 推論結果を画像として描画
#     result_image = results[0].plot()  # 推論結果を描画（numpy配列として返される）

#     # OpenCVを使って画像を保存
#     cv2.imwrite(output_image_path, result_image)
#     print(f"Detection complete. Results saved to: {output_image_path}")

# # サンプル実行
# detect_objects(
#     r"C:\Users\crwba\Rust_Project\object_rustect\image\dog.jpg",
#     r"C:\Users\crwba\Rust_Project\object_rustect\Results\detected_dog.jpg"
# )




# 複数画像

from ultralytics import YOLO
import os
import cv2

def detect_objects(image_path, output_image_path):
    # ファイルパスの確認
    if not os.path.exists(image_path):
        raise FileNotFoundError(f"The specified image file does not exist: {image_path}")
    print(f"Processing image: {image_path}")

    # YOLOモデルをロード
    model = YOLO("yolov8n.pt")  # 小型モデル (yolov8n)

    # 推論を実行
    results = model(image_path)

    # 推論結果を画像として描画
    result_image = results[0].plot()  # 推論結果を描画（numpy配列として返される）

    # OpenCVを使って画像を保存
    cv2.imwrite(output_image_path, result_image)
    print(f"Detection complete. Saved to: {output_image_path}")

def process_folder(input_folder, results_folder):
    # フォルダーが存在するか確認
    if not os.path.exists(input_folder):
        raise FileNotFoundError(f"The specified input folder does not exist: {input_folder}")

    # Resultsフォルダーが存在しない場合は作成
    if not os.path.exists(results_folder):
        os.makedirs(results_folder)

    # フォルダー内のすべての画像ファイルを取得
    image_extensions = (".jpg", ".jpeg", ".png")  # 対応する画像拡張子
    for file_name in os.listdir(input_folder):
        if file_name.lower().endswith(image_extensions):
            input_image_path = os.path.join(input_folder, file_name)
            output_image_path = os.path.join(results_folder, f"detected_{file_name}")

            # 各画像を処理
            detect_objects(input_image_path, output_image_path)

# サンプル実行
process_folder(
    r"C:\Users\crwba\Rust_Project\object_rustect\image",  # 入力フォルダー
    r"C:\Users\crwba\Rust_Project\object_rustect\Results"  # 出力フォルダー
)
