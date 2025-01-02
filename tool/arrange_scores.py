# 機能：プロジェクト全体に存在するarchive_solvesエクセルを検出してスコアを集計する
import glob, os
import pandas as pd
from openpyxl import load_workbook

class ArrangeScores:
    def __init__(self):
        self.scores = []
        self.scores.append(["archive", "score", "total_score"])
        self.extentions_allowance = ["xlsx", "xlsm"]
        self.filename = "archive_solves"
        self.sheet_read_target = "レビュー"
        self.filepath_output = f"{os.path.dirname(os.path.dirname(os.path.abspath(__file__)))}/2025/overviews.xlsx"
        self.sheet_write_target = "問題集結果"
    
    def arrange_scores(self, folder_target):
        filepaths = self.__get_files(folder_target)
        for filepath in filepaths:
            if "df" in locals():
                df = pd.concat([df, self.__get_scores(filepath)])
            else:
                df = self.__get_scores(filepath)
        self.__write_overview(df)

    # ターゲットフォルダ配下のファイルを取得する
    def __get_files(self, folder_target):
        files = []
        for extention in self.extentions_allowance:
            files += glob.glob(f"{folder_target}/**/{self.filename}.{extention}", recursive=True)
        return files

    def __get_scores(self, filepath):
        df = pd.read_excel(filepath, sheet_name=self.sheet_read_target, header=0)
        return df
    
    def __write_overview(self, df):
        book = load_workbook(self.filepath_output, keep_vba=True)
        # 書き込みたいシートを指定 
        # pd.ExcelWriterを使って既存のワークブックに書き込む
        with pd.ExcelWriter(self.filepath_output, engine='openpyxl') as writer:
            writer.book = book
            writer.sheets = {ws.title: ws for ws in book.worksheets}
            
            # データフレームを書き込む
            df.to_excel(writer, sheet_name=self.sheet_write_target, index=False)
            writer.save()
        # データフレームを書き込む 
        # book.save(self.filepath_output)

if __name__ == "__main__":
    arrange_op = ArrangeScores()
    dir_current = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    arrange_op.arrange_scores(f"{dir_current}/2025")



    