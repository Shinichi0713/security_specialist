
import json
import re
import tkinter as tk
from tkinter import filedialog

def get_file_path():
    root = tk.Tk()
    root.withdraw()  # メインウィンドウを表示しないようにする
    file_path = filedialog.askopenfilename()  # ファイルダイアログを開く
    return file_path

def parse_markdown(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        content = file.read()

    # 改行やスペースの数の影響を受けないようにする正規表現パターン
    pattern = re.compile(r'##\s*問題番号\s*###\s*解答\s*<textarea[^>]*>(.*?)`</textarea>`\s*###\s*正誤\s*- \*\*得点:\*\*

\[(.*?)\]

\s*- \*\*配点:\*\*

\[(.*?)\]

', re.DOTALL)
    matches = pattern.findall(content)

    results = []
    for i, match in enumerate(matches, start=1):
        answer, score, total_score = match
        results.append({
            "問題番号": i,
            "解答": answer.strip(),
            "得点": int(score.strip()) if score.strip().isdigit() else 0,
            "配点": int(total_score.strip()) if total_score.strip().isdigit() else 0
        })

    return results

def main():
    file_path = get_file_path()  # ファイルパスを取得
    results = parse_markdown(file_path)

    # JSON形式で出力
    json_output = json.dumps(results, ensure_ascii=False, indent=4)
    print(json_output)

if __name__ == "__main__":
    main()
