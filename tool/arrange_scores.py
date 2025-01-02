# 機能：プロジェクト全体に存在するarchive_solvesエクセルを検出してスコアを集計する
import glob, os
import pandas as pd

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
        filepaths_df = []
        for filepath in filepaths:
            if "df" in locals():
                df_add = self.__get_scores(filepath)
                df = pd.concat([df, df_add])
                filepaths_df.extend([filepath for _ in range(len(df_add))])
            else:
                df = self.__get_scores(filepath)
                filepaths_df.extend([filepath for _ in range(len(df))])
        new_df = pd.DataFrame(filepaths_df, columns=['ファイルパス'], index=None)
        header_to_remove = 'シート番号'
        if header_to_remove in df.columns:
            # ヘッダーを削除
            df = df.drop(columns=[header_to_remove])
        df = pd.concat([df.reset_index(drop=True), new_df.reset_index(drop=True)], axis=1)
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
        # 書き込みたいシートを指定 
        # pd.ExcelWriterを使って既存のワークブックに書き込む
        with pd.ExcelWriter(self.filepath_output, engine='openpyxl') as writer:            
            # データフレームを書き込む
            df.to_excel(writer, sheet_name=self.sheet_write_target, index=False)

if __name__ == "__main__":
    arrange_op = ArrangeScores()
    dir_current = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    arrange_op.arrange_scores(f"{dir_current}/2025")



    