
import json
import re
import tkinter as tk 
from tkinter import filedialog

def parse_markdown(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        content = file.read()

    # 問題ごとの情報を抽出する正規表現パターン
    str_pattern = """
    ##\s*問題番号\s*###\s*解答\s*<textarea[^>]*>(.*?)`</textarea>`\s*###\s*正誤\s*- \*\*得点:\*\*\[(.*?)\]\s*- \*\*配点:\*\*\[(.*?)\]
    """
    pattern = re.compile(str_pattern, re.DOTALL)
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

def get_file_path():
    root = tk.Tk()
    root.withdraw()
    file_path = filedialog.askopenfilename()
    return file_path

def main():
    file_path = get_file_path()  # ここにMarkdownファイルのパスを指定してください
    results = parse_markdown(file_path)

    # JSON形式で出力
    json_output = json.dumps(results, ensure_ascii=False, indent=4)
    print(json_output)

if __name__ == "__main__":
    main()


