# 機能：プロジェクト全体に存在するarchive_solvesエクセルを検出してスコアを集計する
import glob



class ArrangeScores:
    def __init__(self):
        self.scores = []
        self.scores.append(["archive", "score", "total_score"])

